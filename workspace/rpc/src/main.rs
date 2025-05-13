use tracing::error;

use base::config::Config;
use base::database::Database;
use base::error::Error;
use rpc::server::Server;

#[tokio::main]
async fn main() -> Result<(), Error> {
    let _config = Config::load()?;
    let pool = Database::connect_database().await?;

    Server::start_server(pool).await.map_err(|e| {
        error!("{}", e);
        e
    })
}
