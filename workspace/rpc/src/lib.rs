use tonic::Request;
use tonic::Response;
use tonic::Status;

use rpc::GetUserRequest;
use rpc::GetUserResponse;
use rpc::users_service_server::UsersService;
use rpc::users_service_server::UsersServiceServer;

pub mod rpc {
    tonic::include_proto!("example.users.v1.rpc");
}

pub mod types {
    tonic::include_proto!("example.users.v1.types");
}

#[derive(Debug, Default)]
pub struct RpcServer {}

#[tonic::async_trait]
impl UsersService for RpcServer {
    async fn get_user(
        &self,
        request: Request<GetUserRequest>,
    ) -> Result<Response<GetUserResponse>, Status> {
        Ok(tonic::Response::new(GetUserResponse { user: None }))
    }
}

pub async fn start_server() {
    base::connect_database();

    println!("Starting server...");
}
