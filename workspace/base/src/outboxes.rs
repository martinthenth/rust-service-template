mod outbox;
mod outbox_type;

use sea_query::PostgresQueryBuilder;
use sea_query::Query;
use sea_query_binder::SqlxBinder;
use time::OffsetDateTime;
use tracing::instrument;
use uuid::Uuid;

use crate::database::DbExecutor;
use crate::error::Error;
use crate::outboxes::outbox::Outbox;
use crate::outboxes::outbox::OutboxTable;
use crate::outboxes::outbox_type::OutboxType;

#[derive(Debug)]
pub struct CreateOutboxParams {
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
        // TODO: Payload to BYTEA via protobuf
        let id = Uuid::now_v7();
        let timestamp = OffsetDateTime::now_utc();
        let (sql, values) = Query::insert()
            .into_table(OutboxTable::Table)
            .columns([
                OutboxTable::Id,
                OutboxTable::Type,
                OutboxTable::Payload,
                OutboxTable::Timestamp,
            ])
            .values_panic([
                id.into(),
                params.r#type.into(),
                params.payload.into(),
                timestamp.into(),
            ])
            .returning_all()
            .build_sqlx(PostgresQueryBuilder);
        let outbox = sqlx::query_as_with::<_, Outbox, _>(&sql, values)
            .fetch_one(db)
            .await?;

        Ok(outbox)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[meta::data_case]
    async fn test_create_outbox_returns_outbox() {
        let params = CreateOutboxParams {
            r#type: OutboxType::UserCreated,
            payload: vec![1, 2, 3],
        };
        let outbox = Outboxes::create_outbox(&mut *conn, params).await.unwrap();

        assert_eq!(outbox.r#type, OutboxType::UserCreated);
        assert_eq!(outbox.payload, vec![1, 2, 3]);
    }
}
