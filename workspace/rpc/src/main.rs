use init_tracing_opentelemetry::tracing_subscriber_ext;
use tracing::error;

use base::config::Config;
use base::database::Database;
use base::error::Error;
use rpc::server::Server;

#[tokio::main]
async fn main() -> Result<(), Error> {
    let _config = Config::load()?;
    let _guard = tracing_subscriber_ext::init_subscribers()
        .map_err(|e| Error::InternalServer(format!("Failed to initialize Tracer: {e}")))?;
    let pool = Database::connect_database().await?;

    Server::start_server(pool).await.map_err(|e| {
        error!("{}", e);
        e
    })
}
