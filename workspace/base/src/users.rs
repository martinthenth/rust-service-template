pub mod user;

use sea_query::Expr;
use sea_query::PostgresQueryBuilder;
use sea_query::Query;
use sea_query_binder::SqlxBinder;
use sqlx::PgConnection;
use time::OffsetDateTime;
use tracing::instrument;
use uuid::Uuid;

use crate::error::Error;
use crate::users::user::User;
use crate::users::user::UsersTable;

#[derive(Debug, PartialEq)]
pub struct UserCreateParams {
    pub first_name: String,
    pub last_name: String,
}

pub struct Users;

impl Users {
    #[instrument]
    pub async fn get_user_by_id(conn: &mut PgConnection, id: Uuid) -> Result<Option<User>, Error> {
        let (sql, values) = Query::select()
            .from(UsersTable::Table)
            .columns([
                UsersTable::Id,
                UsersTable::FirstName,
                UsersTable::LastName,
                UsersTable::BannedAt,
                UsersTable::CreatedAt,
                UsersTable::UpdatedAt,
                UsersTable::DeletedAt,
            ])
            .and_where(Expr::col(UsersTable::Id).eq(id))
            .build_sqlx(PostgresQueryBuilder);
        let user = sqlx::query_as_with::<_, User, _>(&sql, values)
            .fetch_optional(&mut *conn)
            .await?;

        Ok(user)
    }

    #[instrument]
    pub async fn create_user(
        conn: &mut PgConnection,
        params: UserCreateParams,
    ) -> Result<User, Error> {
        let id = Uuid::now_v7();
        let timestamp = OffsetDateTime::now_utc();
        let (sql, values) = Query::insert()
            .into_table(UsersTable::Table)
            .columns([
                UsersTable::Id,
                UsersTable::FirstName,
                UsersTable::LastName,
                UsersTable::BannedAt,
                UsersTable::CreatedAt,
                UsersTable::UpdatedAt,
                UsersTable::DeletedAt,
            ])
            .values_panic([
                id.into(),
                params.first_name.into(),
                params.last_name.into(),
                None::<OffsetDateTime>.into(),
                timestamp.into(),
                timestamp.into(),
                None::<OffsetDateTime>.into(),
            ])
            .returning_all()
            .build_sqlx(PostgresQueryBuilder);
        let user = sqlx::query_as_with::<_, User, _>(&sql, values)
            .fetch_one(&mut *conn)
            .await?;

        Ok(user)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Factory;

    #[meta::data_case]
    async fn test_create_user_returns_user() {
        let params = UserCreateParams {
            first_name: "John".to_string(),
            last_name: "Doe".to_string(),
        };
        let result = Users::create_user(&mut conn, params).await.unwrap();

        assert_eq!(result.first_name, "John");
        assert_eq!(result.last_name, "Doe");
        assert_eq!(result.banned_at, None);
        assert_eq!(result.created_at, result.updated_at);
        assert_eq!(result.deleted_at, None);
    }

    #[meta::data_case]
    async fn test_get_user_by_id_returns_user() {
        let mut user = User::factory();
        user.first_name = "Jane".to_string();
        user.last_name = "Smith".to_string();
        user.insert(&mut conn).await;

        let result = Users::get_user_by_id(&mut conn, user.id).await.unwrap();

        assert_eq!(result, Some(user));
    }

    #[meta::data_case]
    async fn test_get_user_by_id_does_not_exist_returns_none() {
        let id = Uuid::now_v7();

        let result = Users::get_user_by_id(&mut conn, id).await.unwrap();

        assert_eq!(result, None);
    }
}
