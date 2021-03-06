use {
    crate::{app::Session, Request},
    failure::Fail,
    oauth2::{AsyncCodeTokenRequest, AuthorizationCode, CsrfToken, Scope, TokenResponse},
    serde::Deserialize,
    tide::{Redirect, Response, Result, StatusCode},
};

#[derive(Debug, Deserialize)]
struct AuthRequestQuery {
    code: String,
    state: String,
    scope: String,
}

#[derive(Debug, Deserialize)]
struct UserInfoResponse {
    email: String,
}

pub(super) async fn login(req: Request) -> Result<Redirect<String>> {
    let oauth_client = &req.state().google_oauth_client;

    let (authorize_url, _csrf_state) = oauth_client
        .authorize_url(CsrfToken::new_random)
        .add_scope(Scope::new(
            "https://www.googleapis.com/auth/userinfo.email".to_string(),
        ))
        .url();

    Ok(Redirect::new(authorize_url.to_string()))
}

pub(super) async fn login_authorized(req: Request) -> Result {
    let query: AuthRequestQuery = req.query()?;
    let code = AuthorizationCode::new(query.code);
    let token_req = req.state().google_oauth_client.exchange_code(code);
    let token = token_req
        .request_async(oauth2::reqwest::async_http_client)
        .await
        .map_err(Fail::compat)?;
    let access_token = token.access_token();

    let userinfo: UserInfoResponse = surf::get("https://www.googleapis.com/oauth2/v2/userinfo")
        .set_header(
            surf::http_types::headers::AUTHORIZATION,
            format!("Bearer {}", access_token.secret()),
        )
        .recv_json()
        .await?;

    let session = Session {
        email: userinfo.email,
    };

    let resp: Response = Redirect::new("/").into();
    // FIXME
    // use Response::insert_ext
    // https://github.com/http-rs/tide/commit/7f946a9c9bee84c430dda62ebdf736b287fa0797
    let mut resp: tide::http::Response = resp.into();
    resp.ext_mut().insert(session);
    let resp = resp.into();

    Ok(resp)
}

pub(super) async fn logout(req: Request) -> Result {
    let cookie = req.cookie("session");
    let mut resp = Response::new(StatusCode::Ok);
    if let Some(mut cookie) = cookie {
        cookie.set_path("/");
        resp.remove_cookie(cookie);
    }
    resp.set_body("Goodbye!".to_string());
    Ok(resp)
}
