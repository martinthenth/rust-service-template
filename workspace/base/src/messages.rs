use sea_query::Expr;
use sea_query::PostgresQueryBuilder;
use sea_query::Query;
use sea_query_binder::SqlxBinder;
use time::OffsetDateTime;
use tracing::instrument;
use uuid::Uuid;

use crate::database::DbExecutor;
use crate::error::Error;
use crate::message::Message;
use crate::message::MessagesTable;
use crate::message_type::MessageType;

#[derive(Debug)]
pub struct CreateMessageParams {
    pub id: Uuid,
    pub r#type: MessageType,
    pub payload: Vec<u8>,
    pub timestamp: OffsetDateTime,
}

pub struct Messages;

impl Messages {
    /// Check if a message exists by its ID.
    #[instrument]
    pub async fn get_message_by_id(db: impl DbExecutor<'_>, id: Uuid) -> Result<Message, Error> {
        let (sql, values) = Query::select()
            .from(MessagesTable::Table)
            .columns([
                MessagesTable::Id,
                MessagesTable::Type,
                MessagesTable::Payload,
                MessagesTable::Timestamp,
            ])
            .and_where(Expr::col(MessagesTable::Id).eq(id))
            .build_sqlx(PostgresQueryBuilder);
        let message = sqlx::query_as_with::<_, Message, _>(&sql, values)
            .fetch_one(db)
            .await?;

        Ok(message)
    }

    /// Create a message.
    #[instrument]
    pub async fn create_message(
        db: impl DbExecutor<'_>,
        params: CreateMessageParams,
    ) -> Result<Message, Error> {
        let (sql, values) = Query::insert()
            .into_table(MessagesTable::Table)
            .columns([
                MessagesTable::Id,
                MessagesTable::Type,
                MessagesTable::Payload,
                MessagesTable::Timestamp,
            ])
            .values_panic([
                params.id.into(),
                params.r#type.into(),
                params.payload.into(),
                params.timestamp.into(),
            ])
            .returning_all()
            .build_sqlx(PostgresQueryBuilder);
        let message = sqlx::query_as_with::<_, Message, _>(&sql, values)
            .fetch_one(db)
            .await?;

        Ok(message)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Factory;

    mod get_message_by_id {
        use super::*;

        #[meta::data_case]
        async fn returns_message() {
            let message = Message::insert(&mut *conn, Message::factory()).await;

            let result = Messages::get_message_by_id(&mut *conn, message.id)
                .await
                .unwrap();

            assert_eq!(result, message);
        }
    }

    mod create_message {
        use super::*;

        #[meta::data_case]
        async fn returns_message() {
            let id = Uuid::now_v7();
            let timestamp = OffsetDateTime::now_utc();
            let params = CreateMessageParams {
                id,
                r#type: MessageType::UserCreated,
                payload: vec![1, 2, 3],
                timestamp,
            };
            let message = Messages::create_message(&mut *conn, params).await.unwrap();

            assert_eq!(message.id, id);
            assert_eq!(message.r#type, MessageType::UserCreated);
            assert_eq!(message.payload, vec![1, 2, 3]);
            assert_eq!(message.timestamp, timestamp);
        }
    }
}
