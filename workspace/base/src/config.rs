use dotenvy::from_path;
use std::env::var;
use std::path::Path;
use std::sync::LazyLock;

use crate::error::Error;

pub static CONFIG: LazyLock<Config> = LazyLock::new(Config::default);

#[derive(Debug)]
pub struct Config {
    pub database_url: String,
    pub http_url: String,
    pub grpc_url: String,
    pub cors_origins: Vec<String>,
}

impl Config {
    pub fn load() -> Result<Self, Error> {
        match var("ENV").unwrap_or_default().as_str() {
            "prod" => from_path(Path::new(".env.prod"))
                .map_err(|e| Error::InternalServer(format!("Failed to load .env.prod: {e}")))?,
            // TODO: Load ENV in ctor, then default to dev
            "dev" => from_path(Path::new(".env.dev"))
                .map_err(|e| Error::InternalServer(format!("Failed to load .env.dev: {e}")))?,
            _ => from_path(Path::new(".env.test"))
                .map_err(|e| Error::InternalServer(format!("Failed to load .env.test: {e}")))?,
        }
        from_path(Path::new(".env"))
            .map_err(|e| Error::InternalServer(format!("Failed to load base .env file: {e}")))?;

        Ok(Self::default())
    }
}

impl Default for Config {
    fn default() -> Self {
        Self {
            database_url: var("DATABASE_URL").unwrap(),
            http_url: var("HTTP_URL").unwrap(),
            grpc_url: var("GRPC_URL").unwrap(),
            cors_origins: var("CORS_ORIGINS")
                .unwrap_or_default()
                .split(',')
                .map(String::from)
                .collect(),
        }
    }
}
