use axum::Extension;
use axum::Json;
use axum::Router;
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::routing::get;
use axum_tracing_opentelemetry::middleware::OtelAxumLayer;
use axum_tracing_opentelemetry::middleware::OtelInResponseLayer;
use serde_json::json;
use sqlx::PgPool;
use tokio::net::TcpListener;
use tracing::info;

use base::config::CONFIG;
use base::error::Error;

#[derive(Debug)]
pub struct WebServer {}

impl WebServer {
    pub async fn start_server(pool: PgPool) -> Result<(), Error> {
        let address = &CONFIG.http_url;
        let listener = TcpListener::bind(address)
            .await
            .map_err(|e| Error::InternalServer(format!("Failed to bind HTTP Address: {e}")))?;
        let pool_layer = Extension(pool);
        let server = Router::new()
            .fallback(Self::fallback_json)
            .route("/health", get(Self::health_check))
            .layer(pool_layer)
            .layer(OtelAxumLayer::default())
            .layer(OtelInResponseLayer::default());

        info!("Starting HTTP Server at {address}");

        axum::serve(listener, server)
            .await
            .map_err(|e| Error::InternalServer(format!("Failed to serve HTTP Server: {e}")))
    }

    async fn health_check() -> impl IntoResponse {
        (StatusCode::OK, Json(json!({ "status": "OK" })))
    }

    async fn fallback_json() -> impl IntoResponse {
        (
            StatusCode::NOT_FOUND,
            Json(json!({ "status": "Not Found" })),
        )
    }
}
