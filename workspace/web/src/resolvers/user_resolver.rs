use async_graphql::Result;
use tracing::instrument;
use uuid::Uuid;

use crate::schema::User;
use crate::schema::UserCreateInput;

pub struct UserResolver;

impl UserResolver {
    #[instrument]
    pub async fn create_user(_input: &UserCreateInput) -> Result<Option<User>> {
        Ok(None)
    }

    #[instrument]
    pub async fn user(_id: &Uuid) -> Result<Option<User>> {
        Ok(None)
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
