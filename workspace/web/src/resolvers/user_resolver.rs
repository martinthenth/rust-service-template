use async_graphql::Result;
use tracing::instrument;
use uuid::Uuid;

use crate::schema::CreateUserInput;
use crate::schema::User;
use base::database::DbExecutor;
use base::users::CreateUserParams;
use base::users::Users;

pub struct UserResolver;

impl UserResolver {
    #[instrument]
    pub async fn create_user(
        db: impl DbExecutor<'_>,
        input: CreateUserInput,
    ) -> Result<Option<User>> {
        let params = CreateUserParams {
            first_name: input.first_name,
            last_name: input.last_name,
        };

        match Users::create_user(db, params).await {
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
    pub async fn user(db: impl DbExecutor<'_>, id: Uuid) -> Result<Option<User>> {
        match Users::get_user_by_id(db, id).await {
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
    use base::Factory;
    use base::user::User as BaseUser;

    #[meta::data_case]
    async fn test_create_user_returns_user() {
        let input = CreateUserInput {
            first_name: "John".to_string(),
            last_name: "Doe".to_string(),
        };
        let user = UserResolver::create_user(&mut *conn, input)
            .await
            .unwrap()
            .unwrap();

        assert_eq!(user.first_name, Some("John".to_string()));
        assert_eq!(user.last_name, Some("Doe".to_string()));
    }

    #[meta::data_case]
    async fn test_user_returns_user() {
        let user = BaseUser::insert(&mut *conn, BaseUser::factory()).await;

        let result = UserResolver::user(&mut *conn, user.id).await;

        assert_eq!(
            result,
            Ok(Some(User {
                id: Some(user.id),
                first_name: Some(user.first_name),
                last_name: Some(user.last_name),
                banned_at: user.banned_at,
                created_at: Some(user.created_at),
                updated_at: Some(user.updated_at),
                deleted_at: user.deleted_at
            }))
        );
    }

    #[meta::data_case]
    async fn test_user_does_not_exist_returns_none() {
        let id = Uuid::now_v7();

        let result = UserResolver::user(&mut *conn, id).await;

        assert_eq!(result, Ok(None));
    }
}
