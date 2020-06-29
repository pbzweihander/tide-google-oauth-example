use {serde::Deserialize, std::path::PathBuf, structopt::StructOpt};

#[derive(Debug, StructOpt)]
pub struct Opt {
    #[structopt(parse(from_os_str))]
    pub config: PathBuf,
}

#[derive(Clone, Debug, Deserialize)]
pub struct AppConfig {
    pub secret_key: String,
    pub google_oauth: GoogleOAuthConfig,
}

#[derive(Clone, Debug, Deserialize)]
pub struct GoogleOAuthConfig {
    pub client_id: String,
    pub client_secret: String,
    pub redirect_url: String,
}
