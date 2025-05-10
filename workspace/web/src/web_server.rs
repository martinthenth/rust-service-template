use axum::Json;
use axum::Router;
use axum::http::StatusCode;
use axum::response::IntoResponse;
use serde_json::json;
use tokio::net::TcpListener;
use tracing::info;

#[derive(Debug)]
pub struct WebServer {}

impl WebServer {
    pub async fn start_server() {
        base::connect_database();

        let endpoint_url = "0.0.0.0:4000";
        // TODO: Return application error
        let listener = TcpListener::bind(endpoint_url)
            .await
            .expect("Failed to bind to address");
        let server = Router::new().fallback(Self::fallback_json);

        info!("Running endpoint at http://{}", &endpoint_url);

        axum::serve(listener, server).await.unwrap();
    }

    async fn fallback_json() -> impl IntoResponse {
        (
            StatusCode::NOT_FOUND,
            Json(json!({ "status": "Not Found" })),
        )
    }
}
