use sqlx::Acquire;
use sqlx::Executor;
use sqlx::PgPool;
use sqlx::Postgres;
use sqlx::postgres::PgConnectOptions;
use sqlx::postgres::PgPoolOptions;
use std::str::FromStr;

use crate::config::CONFIG;
use crate::error::Error;

/// A trait for database executors.
pub trait DbExecutor<'a>:
    Acquire<'a, Database = Postgres> + Executor<'a, Database = Postgres>
{
}
impl<'a, T: Acquire<'a, Database = Postgres> + Executor<'a, Database = Postgres>> DbExecutor<'a>
    for T
{
}

pub struct Database {}

impl Database {
    /// Connect to the database.
    pub async fn connect_database() -> Result<PgPool, Error> {
        let database_url = &CONFIG.database_url;
        let options = PgConnectOptions::from_str(database_url)
            .map_err(|e| Error::InternalServer(format!("Failed to parse Database URL: {e}")))?;

        PgPoolOptions::new()
            .max_connections(5)
            .connect_with(options)
            .await
            .map_err(|e| Error::InternalServer(format!("Failed to connect to Database: {e}")))
    }
}
