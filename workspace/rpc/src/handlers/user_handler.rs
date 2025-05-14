use tonic::Status;
use uuid::Uuid;

use crate::users::rpc::GetUserRequest;
use crate::users::rpc::GetUserResponse;
pub use crate::users::rpc::users_server::UsersServer;
use crate::users::types::User;
use base::database::DbExecutor;
use base::users::Users;

pub struct UserHandler;

impl UserHandler {
    /// Get a user.
    pub async fn get_user(
        db: impl DbExecutor<'_>,
        request: GetUserRequest,
    ) -> Result<GetUserResponse, Status> {
        let id =
            Uuid::parse_str(&request.id).map_err(|_| Status::invalid_argument("Invalid id"))?;
        let user = Users::get_user_by_id(db, id)
            .await
            .map_err(|_| Status::internal("Failed service"))?;
        let response = user.map(|u| User {
            id: u.id.to_string(),
            first_name: u.first_name,
            last_name: u.last_name,
            banned_at: u.banned_at.map_or("".to_string(), |dt| dt.to_string()),
            created_at: u.created_at.to_string(),
            updated_at: u.updated_at.to_string(),
            deleted_at: u.deleted_at.map_or("".to_string(), |dt| dt.to_string()),
        });

        Ok(GetUserResponse { user: response })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use base::Factory;
    use base::user::User as BaseUser;

    #[meta::data_case]
    async fn test_get_user_returns_user() {
        let user = BaseUser::insert(&mut *conn, BaseUser::factory()).await;

        let request = GetUserRequest {
            id: user.id.to_string(),
        };
        let response = UserHandler::get_user(&mut *conn, request).await.unwrap();

        assert_eq!(
            response,
            GetUserResponse {
                user: Some(User {
                    id: user.id.to_string(),
                    first_name: user.first_name,
                    last_name: user.last_name,
                    banned_at: user.banned_at.map_or("".to_string(), |s| s.to_string()),
                    created_at: user.created_at.to_string(),
                    updated_at: user.updated_at.to_string(),
                    deleted_at: user.deleted_at.map_or("".to_string(), |s| s.to_string()),
                }),
            }
        );
    }

    #[meta::data_case]
    async fn test_get_user_does_not_exist_returns_none() {
        let id = Uuid::now_v7();
        let request = GetUserRequest { id: id.to_string() };
        let response = UserHandler::get_user(&mut *conn, request).await.unwrap();

        assert_eq!(response.user, None);
    }
}
