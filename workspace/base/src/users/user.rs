use sea_query::Iden;
use sqlx::FromRow;
use time::OffsetDateTime;
use uuid::Uuid;

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
