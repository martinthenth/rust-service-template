use sea_query::Iden;
use sea_query::PostgresQueryBuilder;
use sea_query::Query;
use sea_query_binder::SqlxBinder;
use sqlx::prelude::FromRow;
use time::OffsetDateTime;
use uuid::Uuid;

use crate::Factory;
use crate::database::DbExecutor;
use crate::event_topic::EventTopic;
use crate::event_type::EventType;

#[derive(Debug, FromRow, PartialEq)]
pub struct Event {
    pub id: Uuid,
    pub topic: EventTopic,
    pub r#type: EventType,
    pub payload: Vec<u8>,
    pub timestamp: OffsetDateTime,
}

pub enum EventsTable {
    Table,
    Id,
    Topic,
    Type,
    Payload,
    Timestamp,
}

impl Iden for EventsTable {
    fn unquoted(&self, s: &mut dyn std::fmt::Write) {
        write!(
            s,
            "{}",
            match self {
                Self::Table => "events",
                Self::Id => "id",
                Self::Topic => "topic",
                Self::Type => "type",
                Self::Payload => "payload",
                Self::Timestamp => "timestamp",
            }
        )
        .unwrap();
    }
}

impl Factory for Event {
    #[cfg(feature = "testing")]
    fn factory() -> Self {
        let timestamp = OffsetDateTime::now_utc();

        Event {
            id: Uuid::now_v7(),
            topic: EventTopic::UsersEvents,
            r#type: EventType::UserCreated,
            payload: vec![],
            timestamp,
        }
    }

    #[cfg(feature = "testing")]
    async fn insert(db: impl DbExecutor<'_>, event: Self) -> Self {
        let (sql, values) = Query::insert()
            .into_table(EventsTable::Table)
            .columns([
                EventsTable::Id,
                EventsTable::Topic,
                EventsTable::Type,
                EventsTable::Payload,
                EventsTable::Timestamp,
            ])
            .values_panic([
                event.id.into(),
                event.topic.into(),
                event.r#type.into(),
                event.payload.into(),
                event.timestamp.into(),
            ])
            .returning_all()
            .build_sqlx(PostgresQueryBuilder);

        sqlx::query_as_with::<_, Event, _>(&sql, values)
            .fetch_one(db)
            .await
            .unwrap()
    }
}
