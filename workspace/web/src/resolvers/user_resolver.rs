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
mod tests {}
