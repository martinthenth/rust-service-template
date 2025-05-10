use tracing::info;

use base::database::Database;
use base::error::Error;

#[derive(Debug)]
pub struct BusServer {}

impl BusServer {
    // TODO: Not implemented
    pub async fn start_server() -> Result<(), Error> {
        Database::connect_database().await?;

        info!("Starting Bus Server");

        Ok(())
    }
}
