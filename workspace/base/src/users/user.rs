use sea_query::Iden;
use sea_query::PostgresQueryBuilder;
use sea_query::Query;
use sea_query_binder::SqlxBinder;
use sqlx::FromRow;
use sqlx::PgConnection;
use time::OffsetDateTime;
use uuid::Uuid;

use crate::Factory;

#[derive(Debug, FromRow, PartialEq)]
pub struct User {
    pub id: Uuid,
    pub first_name: String,
    pub last_name: String,
    pub banned_at: Option<OffsetDateTime>,
    pub created_at: OffsetDateTime,
    pub updated_at: OffsetDateTime,
    pub deleted_at: Option<OffsetDateTime>,
}

pub enum UsersTable {
    Table,
    Id,
    FirstName,
    LastName,
    BannedAt,
    CreatedAt,
    UpdatedAt,
    DeletedAt,
}

impl Iden for UsersTable {
    fn unquoted(&self, s: &mut dyn std::fmt::Write) {
        write!(
            s,
            "{}",
            match self {
                Self::Table => "users",
                Self::Id => "id",
                Self::FirstName => "first_name",
                Self::LastName => "last_name",
                Self::BannedAt => "banned_at",
                Self::CreatedAt => "created_at",
                Self::UpdatedAt => "updated_at",
                Self::DeletedAt => "deleted_at",
            }
        )
        .unwrap();
    }
}

impl Factory for User {
    #[cfg(feature = "testing")]
    fn factory() -> Self {
        let timestamp = OffsetDateTime::now_utc();

        User {
            id: Uuid::now_v7(),
            first_name: "John".to_string(),
            last_name: "Doe".to_string(),
            banned_at: None,
            created_at: timestamp,
            updated_at: timestamp,
            deleted_at: None,
        }
    }

    #[cfg(feature = "testing")]
    async fn insert(&self, conn: &mut PgConnection) -> Self {
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
                self.id.into(),
                self.first_name.clone().into(),
                self.last_name.clone().into(),
                None::<OffsetDateTime>.into(),
                self.created_at.into(),
                self.updated_at.into(),
                None::<OffsetDateTime>.into(),
            ])
            .returning_all()
            .build_sqlx(PostgresQueryBuilder);

        sqlx::query_as_with::<_, User, _>(&sql, values)
            .fetch_one(&mut *conn)
            .await
            .unwrap()
    }
}
