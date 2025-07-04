use async_graphql::Context;
use async_graphql::EmptySubscription;
use async_graphql::InputObject;
use async_graphql::Object;
use async_graphql::Result;
use async_graphql::Schema as AGSchema;
use async_graphql::SimpleObject;
use async_graphql::extensions::Tracing;
use sqlx::PgPool;
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

    /// Get the database pool from the context.
    async fn get_pool<'a>(ctx: &'a Context<'_>) -> Result<&'a PgPool> {
        ctx.data::<PgPool>().map_err(|e| {
            Error::InternalServer(format!("Failed to get database pool: {:?}", e)).into()
        })
    }
}

pub type GraphSchema = AGSchema<Query, Mutation, EmptySubscription>;

pub struct Mutation;

#[Object]
impl Mutation {
    /// Create a user.
    async fn create_user(&self, ctx: &Context<'_>, input: CreateUserInput) -> Result<Option<User>> {
        UserResolver::create_user(Schema::get_pool(ctx).await?, input).await
    }
}

pub struct Query;

#[Object]
impl Query {
    /// Get a user.
    async fn user(&self, ctx: &Context<'_>, id: Uuid) -> Result<Option<User>> {
        UserResolver::user(Schema::get_pool(ctx).await?, id).await
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
pub struct CreateUserInput {
    pub first_name: String,
    pub last_name: String,
}
