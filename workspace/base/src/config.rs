use dotenvy::from_path;
use std::env::var;
use std::path::Path;
use std::sync::LazyLock;

pub static CONFIG: LazyLock<Config> = LazyLock::new(Config::default);

#[derive(Debug)]
pub struct Config {
    pub database_url: String,
    pub http_url: String,
    pub grpc_url: String,
}

impl Config {
    fn load_environment_variables() {
        match var("ENV").unwrap_or_default().as_str() {
            "prod" => from_path(Path::new(".env.prod")).expect("Failed to load .env.prod"),
            // TODO: Load ENV in ctor, then default to dev
            "dev" => from_path(Path::new(".env.dev")).expect("Failed to load .env.dev"),
            _ => from_path(Path::new(".env.test")).expect("Failed to load .env.test"),
        }

        from_path(Path::new(".env")).expect("Failed to load base .env file");
    }
}

impl Default for Config {
    fn default() -> Self {
        Self::load_environment_variables();

        Self {
            database_url: var("DATABASE_URL").unwrap(),
            http_url: var("HTTP_URL").unwrap(),
            grpc_url: var("GRPC_URL").unwrap(),
        }
    }
}
