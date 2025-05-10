use tracing::error;

use base::error::Error;
use web::web_server::WebServer;

#[tokio::main]
async fn main() -> Result<(), Error> {
    WebServer::start_server().await.map_err(|e| {
        error!("{}", e);
        e
    })
}
