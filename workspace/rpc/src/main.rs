use tracing::error;

use base::error::Error;
use rpc::rpc_server::RpcServer;

#[tokio::main]
async fn main() -> Result<(), Error> {
    RpcServer::start_server().await.map_err(|e| {
        error!("{}", e);
        e
    })
}
