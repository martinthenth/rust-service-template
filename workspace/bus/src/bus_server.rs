use tracing::info;

use base::error::Error;

#[derive(Debug)]
pub struct BusServer {}

impl BusServer {
    // TODO: Not implemented
    pub async fn start_server() -> Result<(), Error> {
        info!("Starting Bus Server");

        Ok(())
    }
}
