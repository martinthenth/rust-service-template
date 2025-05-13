use sqlx::PgPool;
use tonic::transport::Server as TonicServer;
use tracing::info;

use crate::services::users::UsersServiceServer;
use base::config::CONFIG;
use base::error::Error;

pub mod users {
    tonic::include_proto!("example.users.v1.rpc");
}

pub mod types {
    tonic::include_proto!("example.users.v1.types");
}

pub struct Server {
    pub pool: PgPool,
}

impl Server {
    /// Start the server.
    pub async fn start_server(pool: PgPool) -> Result<(), Error> {
        let address = CONFIG
            .grpc_url
            .parse()
            .map_err(|e| Error::InternalServer(format!("Failed to parse RPC URL: {e}")))?;
        let server = Server { pool };
        let users_service = UsersServiceServer::new(server);

        info!("Starting RPC Server at {address}");

        TonicServer::builder()
            .add_service(users_service)
            .serve(address)
            .await
            .map_err(|e| Error::InternalServer(format!("Failed to serve RPC Server: {e}")))
    }
}
