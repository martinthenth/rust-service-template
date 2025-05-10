use tracing::error;

use base::config::Config;
use base::error::Error;
use rpc::rpc_server::RpcServer;

#[tokio::main]
async fn main() -> Result<(), Error> {
    let _config = Config::load()?;

    RpcServer::start_server().await.map_err(|e| {
        error!("{}", e);
        e
    })
}
