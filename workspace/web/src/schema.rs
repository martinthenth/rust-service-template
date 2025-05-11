use async_graphql::Context;
use async_graphql::EmptySubscription;
use async_graphql::InputObject;
use async_graphql::Object;
use async_graphql::Result;
use async_graphql::Schema as AGSchema;
use async_graphql::SimpleObject;
use async_graphql::extensions::Tracing;
use sqlx::PgPool;
use sqlx::Postgres;
use sqlx::pool::PoolConnection;
use time::OffsetDateTime;
use uuid::Uuid;

use crate::resolvers::user_resolver::UserResolver;
use base::error::Error;

pub struct Schema;

impl Schema {
    /// Create the GraphQL schema.
    pub fn create_schema() -> AGSchema<Query, Mutation, EmptySubscription> {
        AGSchema::build(Query, Mutation, EmptySubscription)
            .extension(Tracing)
            .finish()
    }

    async fn get_conn(ctx: &Context<'_>) -> Result<PoolConnection<Postgres>> {
        ctx.data::<PgPool>()
            .map_err(|e| {
                Error::InternalServer(format!("Failed to get database pool: {:?}", e)).into()
            })?
            .acquire()
            .await
            .map_err(|e| {
                Error::InternalServer(format!("Failed to acquire database connection: {:?}", e))
                    .into()
            })
    }
}

pub type GraphSchema = AGSchema<Query, Mutation, EmptySubscription>;

pub struct Mutation;

#[Object]
impl Mutation {
    /// Create a user.
    async fn create_user(&self, ctx: &Context<'_>, input: UserCreateInput) -> Result<Option<User>> {
        UserResolver::create_user(&mut *Schema::get_conn(ctx).await?, input).await
    }
}

pub struct Query;

#[Object]
impl Query {
    /// Get a user.
    async fn user(&self, ctx: &Context<'_>, id: Uuid) -> Result<Option<User>> {
        UserResolver::user(&mut *Schema::get_conn(ctx).await?, id).await
    }
}

#[derive(Debug, PartialEq, SimpleObject)]
pub struct User {
    pub id: Option<Uuid>,
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub banned_at: Option<OffsetDateTime>,
    pub created_at: Option<OffsetDateTime>,
    pub updated_at: Option<OffsetDateTime>,
    pub deleted_at: Option<OffsetDateTime>,
}

#[derive(Debug, InputObject)]
pub struct UserCreateInput {
    pub first_name: String,
    pub last_name: String,
}
