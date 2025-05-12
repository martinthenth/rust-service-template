use tracing::error;

use base::config::Config;
use base::error::Error;
use rpc::server::Server;

#[tokio::main]
async fn main() -> Result<(), Error> {
    let _config = Config::load()?;

    Server::start_server().await.map_err(|e| {
        error!("{}", e);
        e
    })
}
