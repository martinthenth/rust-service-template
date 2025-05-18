use heck::ToSnakeCase;
use prost::Message;
use sea_query::PostgresQueryBuilder;
use sea_query::Query;
use sea_query_binder::SqlxBinder;
use time::OffsetDateTime;
use tracing::instrument;
use uuid::Uuid;

use crate::common::Envelope;
use crate::database::DbExecutor;
use crate::error::Error;
use crate::outbox::Outbox;
use crate::outbox::OutboxTable;
use crate::outbox_topic::OutboxTopic;
use crate::outbox_type::OutboxType;

#[derive(Debug)]
pub struct CreateOutboxParams {
    pub key: Uuid,
    pub r#type: OutboxType,
    pub payload: Vec<u8>,
}

pub struct Outboxes;

impl Outboxes {
    /// Create an outbox.
    #[instrument]
    pub async fn create_outbox(
        db: impl DbExecutor<'_>,
        params: CreateOutboxParams,
    ) -> Result<Outbox, Error> {
        let id = Uuid::now_v7();
        let topic = OutboxTopic::UsersEvents;
        let timestamp = OffsetDateTime::now_utc();
        let payload = Envelope {
            id: id.to_string(),
            r#type: params.r#type.to_string().to_snake_case(),
            timestamp: timestamp.to_string(),
            payload: params.payload,
        };
        let (sql, values) = Query::insert()
            .into_table(OutboxTable::Table)
            .columns([
                OutboxTable::Id,
                OutboxTable::Topic,
                OutboxTable::Key,
                OutboxTable::Payload,
                OutboxTable::Timestamp,
            ])
            .values_panic([
                id.into(),
                topic.into(),
                params.key.into(),
                payload.encode_to_vec().into(),
                timestamp.into(),
            ])
            .returning_all()
            .build_sqlx(PostgresQueryBuilder);
        let outbox = sqlx::query_as_with::<_, Outbox, _>(&sql, values)
            .fetch_one(db)
            .await?;

        Ok(outbox)
    }

    #[cfg(test)]
    pub async fn list_outboxes(db: impl DbExecutor<'_>) -> Result<Vec<Outbox>, Error> {
        let (sql, values) = Query::select()
            .from(OutboxTable::Table)
            .columns([
                OutboxTable::Id,
                OutboxTable::Topic,
                OutboxTable::Key,
                OutboxTable::Payload,
                OutboxTable::Timestamp,
            ])
            .build_sqlx(PostgresQueryBuilder);
        let outboxes = sqlx::query_as_with::<_, Outbox, _>(&sql, values)
            .fetch_all(db)
            .await?;

        Ok(outboxes)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    mod create_outbox {
        use super::*;

        #[meta::data_case]
        async fn returns_outbox() {
            let key = Uuid::now_v7();
            let params = CreateOutboxParams {
                key,
                r#type: OutboxType::UserCreated,
                payload: vec![1, 2, 3],
            };
            let outbox = Outboxes::create_outbox(&mut *conn, params).await.unwrap();

            assert_eq!(outbox.topic, OutboxTopic::UsersEvents);
            assert_eq!(outbox.key, key);

            let payload = Envelope::decode(outbox.payload.as_slice()).unwrap();

            assert_eq!(payload.r#type, OutboxType::UserCreated.to_string());
            assert_eq!(payload.payload, vec![1, 2, 3]);
        }
    }
}
