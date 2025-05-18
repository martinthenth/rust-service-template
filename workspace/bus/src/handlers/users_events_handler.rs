use prost::Message;
use sqlx::types::Uuid;
use time::OffsetDateTime;
use time::format_description::well_known::Iso8601;
use tracing::instrument;

use crate::common::Envelope;
use crate::users::events::UserCreated;
use base::database::DbExecutor;
use base::error::Error;
use base::events::CreateEventParams;
use base::events::Events;

pub struct UsersEventsHandler;

impl UsersEventsHandler {
    #[instrument]
    pub async fn handle_message(db: impl DbExecutor<'_>, envelope: Envelope) -> Result<(), Error> {
        let event_id = Uuid::parse_str(&envelope.id)
            .map_err(|e| Error::InternalServer(format!("Failed to parse UUID: {}", e)))?;
        let mut tx = db.begin().await?;

        if Events::get_event_by_id(&mut *tx, event_id).await?.is_some() {
            return Ok(());
        }

        let user_created = UserCreated::decode(envelope.payload.as_slice())
            .map_err(|e| Error::InternalServer(format!("Failed to decode payload: {}", e)))?;
        let timestamp = OffsetDateTime::parse(&envelope.timestamp, &Iso8601::DEFAULT)
            .map_err(|e| Error::InternalServer(format!("Failed to parse timestamp: {}", e)))?;
        let params = CreateEventParams {
            id: event_id,
            r#type: envelope.r#type.into(),
            payload: envelope.payload,
            timestamp,
        };

        println!("params: {:?}", params);
        println!("user_created: {:?}", user_created);
        Events::create_event(&mut *tx, params).await?;
        tx.commit().await?;

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use base::Factory;

    mod handle_message {
        use super::*;
        use base::event::Event;

        #[meta::data_case]
        async fn returns_ok() {
            let envelope = Envelope {
                id: "0196e597-025a-70d0-a007-db67265b8d1d".to_string(),
                r#type: "user_created".to_string(),
                payload: vec![],
                timestamp: "2021-01-01T00:00:00Z".to_string(),
            };

            let result = UsersEventsHandler::handle_message(&mut *conn, envelope)
                .await
                .unwrap();

            assert_eq!(result, ());

            let events = Events::list_events(&mut *conn).await.unwrap();

            assert_eq!(events.len(), 1);
        }

        #[meta::data_case]
        async fn already_exists_returns_ok() {
            let event = Event::insert(&mut *conn, Event::factory()).await;

            let envelope = Envelope {
                id: event.id.to_string(),
                r#type: "user_created".to_string(),
                payload: vec![],
                timestamp: event.timestamp.to_string(),
            };

            let result = UsersEventsHandler::handle_message(&mut *conn, envelope)
                .await
                .unwrap();

            assert_eq!(result, ());

            let events = Events::list_events(&mut *conn).await.unwrap();

            assert_eq!(events.len(), 1);
        }
    }
}
