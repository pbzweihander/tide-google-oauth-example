mod api;
mod app;
mod config;
mod google_oauth;

use {
    crate::{
        api::attach_apis,
        app::AppState,
        config::{AppConfig, Opt},
    },
    anyhow::Result,
    structopt::StructOpt,
    tide_secure_cookie_session::SecureCookieSessionMiddleware,
};

pub use crate::app::*;

#[async_std::main]
async fn main() -> Result<()> {
    tide::log::start();

    let opt = Opt::from_args();
    let mut base_config = configer::Config::default();
    base_config.merge(configer::File::from(opt.config))?;
    let config: AppConfig = base_config.try_into()?;

    let session_middleware =
        SecureCookieSessionMiddleware::<Session>::new(config.secret_key.as_bytes().to_vec());
    let state = AppState::from_config(config)?;

    let mut app = tide::with_state(state);
    app.middleware(session_middleware);
    attach_apis(&mut app);
    app.listen("0.0.0.0:8080").await?;

    Ok(())
}
