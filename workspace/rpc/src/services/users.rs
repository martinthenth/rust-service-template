use base::users::Users;
use tonic::Request;
use tonic::Response;
use tonic::Status;
use uuid::Uuid;

use crate::server::Server;
use crate::server::types::User;
use crate::server::users::GetUserRequest;
use crate::server::users::GetUserResponse;
use crate::server::users::users_service_server::UsersService;
pub use crate::server::users::users_service_server::UsersServiceServer;

#[tonic::async_trait]
impl UsersService for Server {
    async fn get_user(
        &self,
        request: Request<GetUserRequest>,
    ) -> Result<Response<GetUserResponse>, Status> {
        let params = request.into_inner();
        let id =
            Uuid::parse_str(&params.id).map_err(|_| Status::invalid_argument("Invalid user id"))?;
        let user = Users::get_user_by_id(&self.pool, id)
            .await
            .map_err(|_| Status::internal("Unknown error"))?
            .ok_or_else(|| Status::not_found("User not found"))?;

        Ok(Response::new(GetUserResponse {
            user: Some(User {
                id: user.id.to_string(),
                first_name: user.first_name,
                last_name: user.last_name,
                banned_at: user.banned_at.map_or("".to_string(), |s| s.to_string()),
                created_at: user.created_at.to_string(),
                updated_at: user.updated_at.to_string(),
                deleted_at: user.deleted_at.map_or("".to_string(), |s| s.to_string()),
            }),
        }))
    }
}

#[cfg(test)]
mod tests {
    // use super::*;
    // use base::Factory;
    // use base::users::user::User;

    #[meta::data_case]
    async fn test_get_user_returns_user() {
        // let user = User::factory().insert(&mut conn).await;
        // let server = Server::new(conn);

        // let request = Request::new(GetUserRequest {
        //     id: user.id.to_string(),
        // });
        // let result = server.get_user(request).await.unwrap();

        // // TODO: Implement this
        // println!("result: {:?}", result);
    }
}
