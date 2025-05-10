use tracing::error;

use base::error::Error;
use bus::bus_server::BusServer;

#[tokio::main]
async fn main() -> Result<(), Error> {
    BusServer::start_server().await.map_err(|e| {
        error!("{}", e);
        e
    })
}
