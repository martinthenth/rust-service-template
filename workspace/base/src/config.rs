use dotenvy::from_path;
use std::env::var;
use std::path::Path;
use std::path::PathBuf;
use std::sync::LazyLock;

use crate::error::Error;

pub static CONFIG: LazyLock<Config> = LazyLock::new(Config::default);

#[derive(Debug)]
pub struct Config {
    pub database_url: String,
    pub http_url: String,
    pub grpc_url: String,
    pub kafka_url: String,
    pub kafka_group: String,
    pub kafka_topic: String,
    pub cors_origins: Vec<String>,
}

impl Config {
    pub fn load() -> Result<Self, Error> {
        let env_file = match var("ENV").unwrap_or_default().as_str() {
            "prod" => ".env.prod",
            "test" => ".env.test",
            _ => ".env.dev",
        };

        from_path(Self::find_env_file(env_file))
            .map_err(|e| Error::InternalServer(format!("Failed to load {env_file}: {e}")))?;

        from_path(Self::find_env_file(".env"))
            .map_err(|e| Error::InternalServer(format!("Failed to load base .env file: {e}")))?;

        Ok(Self::default())
    }

    fn find_env_file(filename: &str) -> PathBuf {
        let current_path = Path::new(filename);
        if current_path.exists() {
            current_path.to_path_buf()
        } else {
            Path::new("..").join(filename)
        }
    }
}

impl Default for Config {
    fn default() -> Self {
        Self {
            database_url: var("DATABASE_URL").unwrap(),
            http_url: var("HTTP_URL").unwrap(),
            grpc_url: var("GRPC_URL").unwrap(),
            kafka_url: var("KAFKA_URL").unwrap(),
            kafka_group: var("KAFKA_GROUP").unwrap(),
            kafka_topic: var("KAFKA_TOPIC").unwrap(),
            cors_origins: var("CORS_ORIGINS")
                .unwrap_or_default()
                .split(',')
                .map(String::from)
                .collect(),
        }
    }
}
