use {
    crate::{config::AppConfig, google_oauth::make_client},
    anyhow::Result,
    oauth2::basic::BasicClient,
    serde::{Deserialize, Serialize},
};

pub type App = tide::Server<AppState>;
pub type Request = tide::Request<AppState>;

#[derive(Debug)]
pub struct AppState {
    pub config: AppConfig,
    pub google_oauth_client: BasicClient,
}

impl AppState {
    pub fn from_config(config: AppConfig) -> Result<Self> {
        let google_oauth_client = make_client(&config.google_oauth)?;
        Ok(Self {
            config,
            google_oauth_client,
        })
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Session {
    pub email: String,
}
