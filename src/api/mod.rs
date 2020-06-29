mod auth;

use {
    crate::{app::Session, App, Request},
    tide::{Redirect, Response, Result, StatusCode},
};

pub fn attach_apis(app: &mut App) {
    app.at("/").get(index);
    app.at("logout/").get(auth::logout);

    let mut login = app.at("login/");
    login.at("/").get(auth::login);
    login.at("authorized/").get(auth::login_authorized);
}

macro_rules! session {
    ($req:expr) => {{
        let session = $req.ext::<Session>();
        if session.is_none() {
            return Ok(Redirect::new("/login/").into());
        }
        session.unwrap()
    }};
}

async fn index(req: Request) -> Result {
    let session = session!(req);
    let body = format!(
        r#"<!DOCTYPE html>
<html>
<body>
<h1>Hello, world!</h1>
<p>Hi, {}!</p>
</body>
</html>
"#,
        session.email,
    );
    let mut resp = Response::new(StatusCode::Ok);
    resp.set_body(body);
    resp.insert_header(tide::http::headers::CONTENT_TYPE, tide::http::mime::HTML);
    Ok(resp)
}
