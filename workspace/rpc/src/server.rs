use tonic::Request;
use tonic::Response;
use tonic::Status;
use tonic::transport::Server as TonicServer;
use tracing::info;

use crate::server::types::User;
use crate::server::users::GetUserRequest;
use crate::server::users::GetUserResponse;
use crate::server::users::users_service_server::UsersService;
use crate::server::users::users_service_server::UsersServiceServer;
use base::config::CONFIG;
use base::error::Error;

pub mod users {
    tonic::include_proto!("example.users.v1.rpc");
}

pub mod types {
    tonic::include_proto!("example.users.v1.types");
}

#[derive(Debug, Default)]
pub struct Server {}

impl Server {
    /// Start the server.
    pub async fn start_server() -> Result<(), Error> {
        let address = CONFIG
            .grpc_url
            .parse()
            .map_err(|e| Error::InternalServer(format!("Failed to parse RPC URL: {e}")))?;
        let server = Server::default();
        let users_service = UsersServiceServer::new(server);

        info!("Starting RPC Server at {address}");

        TonicServer::builder()
            .add_service(users_service)
            .serve(address)
            .await
            .map_err(|e| Error::InternalServer(format!("Failed to serve RPC Server: {e}")))
    }
}

#[tonic::async_trait]
impl UsersService for Server {
    async fn get_user(
        &self,
        request: Request<GetUserRequest>,
    ) -> Result<Response<GetUserResponse>, Status> {
        println!("request: {:?}", request.into_inner());

        Ok(Response::new(GetUserResponse {
            user: Some(User {
                id: "1".to_string(),
                first_name: "John".to_string(),
                last_name: "Doe".to_string(),
                banned_at: "213".to_string(),
                created_at: "".to_string(),
                updated_at: "".to_string(),
                deleted_at: "".to_string(),
            }),
        }))
    }
}
