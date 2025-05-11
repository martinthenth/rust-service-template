use async_graphql::Result;
use sqlx::PgConnection;
use tracing::instrument;
use uuid::Uuid;

use crate::schema::User;
use crate::schema::UserCreateInput;
use base::users::UserCreateParams;
use base::users::Users;
pub struct UserResolver;

impl UserResolver {
    #[instrument]
    pub async fn create_user(
        conn: &mut PgConnection,
        input: UserCreateInput,
    ) -> Result<Option<User>> {
        let params = UserCreateParams {
            first_name: input.first_name,
            last_name: input.last_name,
        };

        match Users::create_user(conn, params).await {
            Ok(user) => Ok(Some(User {
                id: Some(user.id),
                first_name: Some(user.first_name),
                last_name: Some(user.last_name),
                banned_at: user.banned_at,
                created_at: Some(user.created_at),
                updated_at: Some(user.updated_at),
                deleted_at: user.deleted_at,
            })),
            Err(error) => Err(error.into()),
        }
    }

    #[instrument]
    pub async fn user(conn: &mut PgConnection, id: Uuid) -> Result<Option<User>> {
        match Users::get_user_by_id(conn, id).await {
            Ok(Some(user)) => Ok(Some(User {
                id: Some(user.id),
                first_name: Some(user.first_name),
                last_name: Some(user.last_name),
                banned_at: user.banned_at,
                created_at: Some(user.created_at),
                updated_at: Some(user.updated_at),
                deleted_at: user.deleted_at,
            })),
            Ok(None) => Ok(None),
            Err(error) => Err(error.into()),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[meta::data_case]
    async fn test_create_user() {
        let input = UserCreateInput {
            first_name: "John".to_string(),
            last_name: "Doe".to_string(),
        };
        let result = UserResolver::create_user(&mut *conn, input).await;

        assert_eq!(result, Ok(None));
    }

    #[meta::data_case]
    async fn test_user() {
        let id = Uuid::new_v4();

        let result = UserResolver::user(&mut *conn, id).await;

        assert_eq!(result, Ok(None));
    }
}
