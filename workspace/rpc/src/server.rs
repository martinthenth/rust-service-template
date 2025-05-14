use sqlx::PgPool;
use tonic::Request;
use tonic::Response;
use tonic::Status;
use tonic::transport::Server as TonicServer;
use tracing::info;

use crate::handlers::user_handler::UserHandler;
use crate::server::users::GetUserRequest;
use crate::server::users::GetUserResponse;
use crate::server::users::users_server::Users;
pub use crate::server::users::users_server::UsersServer;
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
        let users_service = UsersServer::new(server);

        info!("Starting RPC Server at {address}");

        TonicServer::builder()
            .add_service(users_service)
            .serve(address)
            .await
            .map_err(|e| Error::InternalServer(format!("Failed to serve RPC Server: {e}")))
    }
}

#[tonic::async_trait]
impl Users for Server {
    async fn get_user(
        &self,
        request: Request<GetUserRequest>,
    ) -> Result<Response<GetUserResponse>, Status> {
        Ok(Response::new(
            UserHandler::get_user(&self.pool, request.into_inner()).await?,
        ))
    }
}
