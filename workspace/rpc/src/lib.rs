use tonic::Request;
use tonic::Response;
use tonic::Status;
use tonic::transport::Server;

use rpc::GetUserRequest;
use rpc::GetUserResponse;
use rpc::users_service_server::UsersService;
use rpc::users_service_server::UsersServiceServer;
use types::User;

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
        println!("request: {:?}", request.into_inner());

        Ok(tonic::Response::new(GetUserResponse {
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

pub async fn start_server() {
    base::connect_database();

    let address = "[::1]:50051".parse().unwrap();
    let server = RpcServer::default();

    Server::builder()
        .add_service(UsersServiceServer::new(server))
        .serve(address)
        .await
        .unwrap();
}
