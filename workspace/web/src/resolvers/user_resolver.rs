use async_graphql::Result;
use tracing::instrument;
use uuid::Uuid;

use crate::schema::User;
use crate::schema::UserCreateInput;
use base::users::UserCreateParams;
use base::users::Users;
pub struct UserResolver;

impl UserResolver {
    #[instrument]
    pub async fn create_user(input: &UserCreateInput) -> Result<Option<User>> {
        let params = UserCreateParams {
            first_name: input.first_name.clone(),
            last_name: input.last_name.clone(),
        };

        match Users::create_user(&params).await {
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
    pub async fn user(id: &Uuid) -> Result<Option<User>> {
        match Users::get_user_by_id(id).await {
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

    #[tokio::test]
    async fn test_create_user() {
        let input = UserCreateInput {
            first_name: "John".to_string(),
            last_name: "Doe".to_string(),
        };
        let result = UserResolver::create_user(&input).await;

        assert_eq!(result, Ok(None));
    }

    #[tokio::test]
    async fn test_user() {
        let id = Uuid::new_v4();

        let result = UserResolver::user(&id).await;

        assert_eq!(result, Ok(None));
    }
}
