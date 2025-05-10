use sqlx::ConnectOptions;
use sqlx::PgPool;
use sqlx::postgres::PgConnectOptions;
use sqlx::postgres::PgPoolOptions;
use std::str::FromStr;

use crate::config::CONFIG;
use crate::error::Error;

pub struct Database {}

impl Database {
    /// Connect to the database.
    pub async fn connect_database() -> Result<PgPool, Error> {
        let database_url = &CONFIG.database_url;
        let options = PgConnectOptions::from_str(database_url)
            .map_err(|e| Error::InternalServer(format!("Failed to parse Database URL: {e}")))?
            // TODO: Is this necessary?
            .log_statements(tracing::log::LevelFilter::Debug);

        PgPoolOptions::new()
            .max_connections(5)
            .connect_with(options)
            .await
            .map_err(|e| Error::InternalServer(format!("Failed to connect to Database: {e}")))
    }
}
