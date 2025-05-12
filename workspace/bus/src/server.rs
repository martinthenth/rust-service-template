use tracing::info;

use base::error::Error;

#[derive(Debug)]
pub struct Server {}

impl Server {
    // TODO: Not implemented
    pub async fn start_server() -> Result<(), Error> {
        info!("Starting Bus Server");

        Ok(())
    }
}
