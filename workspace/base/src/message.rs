use sea_query::Iden;
use sea_query::PostgresQueryBuilder;
use sea_query::Query;
use sea_query_binder::SqlxBinder;
use sqlx::prelude::FromRow;
use time::OffsetDateTime;
use uuid::Uuid;

use crate::Factory;
use crate::database::DbExecutor;
use crate::message_type::MessageType;

#[derive(Debug, FromRow, PartialEq)]
pub struct Message {
    pub id: Uuid,
    pub r#type: MessageType,
    pub payload: Vec<u8>,
    pub timestamp: OffsetDateTime,
}

pub enum MessagesTable {
    Table,
    Id,
    Type,
    Payload,
    Timestamp,
}

impl Iden for MessagesTable {
    fn unquoted(&self, s: &mut dyn std::fmt::Write) {
        write!(
            s,
            "{}",
            match self {
                Self::Table => "outbox",
                Self::Id => "id",
                Self::Type => "type",
                Self::Payload => "payload",
                Self::Timestamp => "timestamp",
            }
        )
        .unwrap();
    }
}

impl Factory for Message {
    #[cfg(feature = "testing")]
    fn factory() -> Self {
        let timestamp = OffsetDateTime::now_utc();

        Message {
            id: Uuid::now_v7(),
            r#type: MessageType::UserCreated,
            payload: vec![],
            timestamp,
        }
    }

    #[cfg(feature = "testing")]
    async fn insert(db: impl DbExecutor<'_>, message: Self) -> Self {
        let (sql, values) = Query::insert()
            .into_table(MessagesTable::Table)
            .columns([
                MessagesTable::Id,
                MessagesTable::Type,
                MessagesTable::Payload,
                MessagesTable::Timestamp,
            ])
            .values_panic([
                message.id.into(),
                message.r#type.into(),
                message.payload.into(),
                message.timestamp.into(),
            ])
            .returning_all()
            .build_sqlx(PostgresQueryBuilder);

        sqlx::query_as_with::<_, Message, _>(&sql, values)
            .fetch_one(db)
            .await
            .unwrap()
    }
}
