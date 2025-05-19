use sea_query::Expr;
use sea_query::PostgresQueryBuilder;
use sea_query::Query;
use sea_query_binder::SqlxBinder;
use time::OffsetDateTime;
use tracing::instrument;
use uuid::Uuid;

use crate::database::DbExecutor;
use crate::error::Error;
use crate::event::Event;
use crate::event::EventsTable;
use crate::event_type::EventType;

#[derive(Debug)]
pub struct CreateEventParams {
    pub id: Uuid,
    pub r#type: EventType,
    pub payload: Vec<u8>,
    pub timestamp: OffsetDateTime,
}

pub struct Events;

impl Events {
    /// Get an event by its ID.
    #[instrument]
    pub async fn get_event_by_id(
        db: impl DbExecutor<'_>,
        id: Uuid,
    ) -> Result<Option<Event>, Error> {
        let (sql, values) = Query::select()
            .from(EventsTable::Table)
            .columns([
                EventsTable::Id,
                EventsTable::Type,
                EventsTable::Payload,
                EventsTable::Timestamp,
            ])
            .and_where(Expr::col(EventsTable::Id).eq(id))
            .build_sqlx(PostgresQueryBuilder);
        let event = sqlx::query_as_with::<_, Event, _>(&sql, values)
            .fetch_optional(db)
            .await?;

        Ok(event)
    }

    /// Create an event.
    #[instrument]
    pub async fn create_event(
        db: impl DbExecutor<'_>,
        params: CreateEventParams,
    ) -> Result<Event, Error> {
        let (sql, values) = Query::insert()
            .into_table(EventsTable::Table)
            .columns([
                EventsTable::Id,
                EventsTable::Type,
                EventsTable::Payload,
                EventsTable::Timestamp,
            ])
            .values_panic([
                params.id.into(),
                params.r#type.into(),
                params.payload.into(),
                params.timestamp.into(),
            ])
            .returning_all()
            .build_sqlx(PostgresQueryBuilder);
        let event = sqlx::query_as_with::<_, Event, _>(&sql, values)
            .fetch_one(db)
            .await?;

        Ok(event)
    }

    #[cfg(feature = "testing")]
    pub async fn list_events(db: impl DbExecutor<'_>) -> Result<Vec<Event>, Error> {
        let (sql, values) = Query::select()
            .from(EventsTable::Table)
            .columns([
                EventsTable::Id,
                EventsTable::Type,
                EventsTable::Payload,
                EventsTable::Timestamp,
            ])
            .build_sqlx(PostgresQueryBuilder);
        let events = sqlx::query_as_with::<_, Event, _>(&sql, values)
            .fetch_all(db)
            .await?;

        Ok(events)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Factory;

    mod get_event_by_id {
        use super::*;

        #[meta::data_case]
        async fn returns_event() {
            let event = Event::insert(&mut *conn, Event::factory()).await;

            let result = Events::get_event_by_id(&mut *conn, event.id).await.unwrap();

            assert_eq!(result, Some(event));
        }
    }

    mod create_event {
        use super::*;

        #[meta::data_case]
        async fn returns_event() {
            let id = Uuid::now_v7();
            let timestamp = OffsetDateTime::now_utc();
            let params = CreateEventParams {
                id,
                r#type: EventType::UserCreated,
                payload: vec![1, 2, 3],
                timestamp,
            };
            let event = Events::create_event(&mut *conn, params).await.unwrap();

            assert_eq!(event.id, id);
            assert_eq!(event.r#type, EventType::UserCreated);
            assert_eq!(event.payload, vec![1, 2, 3]);
        }
    }
}
