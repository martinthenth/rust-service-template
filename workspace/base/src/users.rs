use time::OffsetDateTime;
use tracing::instrument;
use uuid::Uuid;

use crate::error::Error;

#[derive(Debug, PartialEq)]
pub struct User {
    pub id: Uuid,
    pub first_name: String,
    pub last_name: String,
    pub banned_at: Option<OffsetDateTime>,
    pub created_at: OffsetDateTime,
    pub updated_at: OffsetDateTime,
    pub deleted_at: Option<OffsetDateTime>,
}

#[derive(Debug, PartialEq)]
pub struct UserCreateParams {
    pub first_name: String,
    pub last_name: String,
}

pub struct Users;

impl Users {
    #[instrument]
    pub async fn get_user_by_id(id: &Uuid) -> Result<Option<User>, Error> {
        Ok(None)
    }

    #[instrument]
    pub async fn create_user(params: &UserCreateParams) -> Result<User, Error> {
        let id = Uuid::now_v7();
        let timestamp = OffsetDateTime::now_utc();

        Ok(User {
            id,
            first_name: params.first_name.clone(),
            last_name: params.last_name.clone(),
            banned_at: None,
            created_at: timestamp,
            updated_at: timestamp,
            deleted_at: None,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_create_user() {
        let params = UserCreateParams {
            first_name: "John".to_string(),
            last_name: "Doe".to_string(),
        };
        let result = Users::create_user(&params).await.unwrap();

        assert_eq!(result.first_name, "John");
        assert_eq!(result.last_name, "Doe");
        assert_eq!(result.banned_at, None);
        assert_eq!(result.created_at, result.updated_at);
        assert_eq!(result.deleted_at, None);
    }

    #[tokio::test]
    async fn test_get_user_by_id() {
        let id = Uuid::new_v4();
        let result = Users::get_user_by_id(&id).await.unwrap();

        assert_eq!(result, None);
    }
}
