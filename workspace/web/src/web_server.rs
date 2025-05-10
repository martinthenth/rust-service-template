use axum::Extension;
use axum::Json;
use axum::Router;
use axum::http::StatusCode;
use axum::response::IntoResponse;
use serde_json::json;
use tokio::net::TcpListener;
use tracing::info;

use base::config::CONFIG;
use base::database::Database;
use base::error::Error;

#[derive(Debug)]
pub struct WebServer {}

impl WebServer {
    pub async fn start_server() -> Result<(), Error> {
        let pool = Database::connect_database().await?;
        let address = &CONFIG.http_url;
        let listener = TcpListener::bind(address)
            .await
            .map_err(|e| Error::InternalServer(format!("Failed to bind HTTP Address: {e}")))?;
        let pool_layer = Extension(pool);
        let server = Router::new()
            .fallback(Self::fallback_json)
            .layer(pool_layer);

        info!("Starting HTTP Server at {address}");

        axum::serve(listener, server)
            .await
            .map_err(|e| Error::InternalServer(format!("Failed to serve HTTP Server: {e}")))
    }

    async fn fallback_json() -> impl IntoResponse {
        (
            StatusCode::NOT_FOUND,
            Json(json!({ "status": "Not Found" })),
        )
    }
}
