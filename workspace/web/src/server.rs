use async_graphql::http::GraphiQLSource;
use async_graphql_axum::GraphQLRequest;
use async_graphql_axum::GraphQLResponse;
use axum::Extension;
use axum::Json;
use axum::Router;
use axum::extract::State;
use axum::http::HeaderMap;
use axum::http::Method;
use axum::http::StatusCode;
use axum::http::header;
use axum::http::header::HeaderValue;
use axum::response::Html;
use axum::response::IntoResponse;
use axum::routing::get;
use axum::routing::post;
use axum_tracing_opentelemetry::middleware::OtelAxumLayer;
use axum_tracing_opentelemetry::middleware::OtelInResponseLayer;
use serde_json::json;
use sqlx::PgPool;
use tokio::net::TcpListener;
use tower_http::cors::CorsLayer;
use tracing::info;

use crate::schema::GraphSchema;
use crate::schema::Schema;
use base::config::CONFIG;
use base::error::Error;

#[derive(Debug)]
pub struct Server {}

impl Server {
    pub async fn start_server(pool: PgPool) -> Result<(), Error> {
        let address = &CONFIG.http_url;
        let listener = TcpListener::bind(address)
            .await
            .map_err(|e| Error::InternalServer(format!("Failed to bind HTTP Address: {e}")))?;
        let schema = Schema::create_schema();
        let cors_headers = [
            header::ACCEPT,
            header::ACCEPT_LANGUAGE,
            header::CONTENT_LANGUAGE,
            header::CONTENT_TYPE,
        ];
        let cors_methods = [
            Method::GET,
            Method::POST,
            Method::PUT,
            Method::PATCH,
            Method::DELETE,
            Method::HEAD,
            Method::OPTIONS,
        ];
        let cors_origins = CONFIG
            .cors_origins
            .iter()
            .map(|s| s.parse().unwrap())
            .collect::<Vec<HeaderValue>>();
        let cors_layer = CorsLayer::new()
            .allow_credentials(true)
            .allow_headers(cors_headers)
            .allow_methods(cors_methods)
            .allow_origin(cors_origins);
        let pool_layer = Extension(pool);
        let server = Router::new()
            .route("/", get(Self::graphiql_html))
            .route("/graph", post(Self::graphql_json))
            .route("/health", get(Self::health_check))
            .fallback(Self::fallback_json)
            .with_state(schema)
            .layer(cors_layer)
            .layer(pool_layer)
            .layer(OtelAxumLayer::default())
            .layer(OtelInResponseLayer);

        info!("Starting HTTP Server at {address}");

        axum::serve(listener, server)
            .await
            .map_err(|e| Error::InternalServer(format!("Failed to serve HTTP Server: {e}")))
    }

    async fn graphiql_html() -> impl IntoResponse {
        Html(GraphiQLSource::build().endpoint("/graph").finish())
    }

    async fn graphql_json(
        State(schema): State<GraphSchema>,
        Extension(pool): Extension<PgPool>,
        headers: HeaderMap,
        request: GraphQLRequest,
    ) -> GraphQLResponse {
        let request = request.into_inner();

        if let Some(token) = headers
            .get("authorization")
            .and_then(|value| value.to_str().ok())
        {
            info!("Auth token: {token}");
        }

        schema.execute(request.data(pool)).await.into()
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
