use tracing::error;

use base::config::Config;
use base::error::Error;
use bus::bus_server::BusServer;

#[tokio::main]
async fn main() -> Result<(), Error> {
    let _config = Config::load()?;

    BusServer::start_server().await.map_err(|e| {
        error!("{}", e);
        e
    })
}
