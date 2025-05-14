mod message;

use time::OffsetDateTime;
use tracing::instrument;
use uuid::Uuid;

use crate::database::DbExecutor;
use crate::error::Error;
use crate::messages::message::Message;

#[derive(Debug)]
pub struct CreateMessageParams {
    pub typx: String,
    pub payload: Vec<u8>,
}

pub struct Messages;

impl Messages {
    /// Check if a message exists by its ID.
    #[instrument]
    pub async fn get_message_by_id(db: impl DbExecutor<'_>, id: Uuid) -> Result<bool, Error> {
        Ok(true)
    }

    /// Create a message.
    #[instrument]
    pub async fn create_message(params: CreateMessageParams) -> Result<Message, Error> {
        Ok(Message {
            id: Uuid::now_v7(),
            typx: params.typx,
            payload: params.payload,
            timestamp: OffsetDateTime::now_utc(),
        })
    }
}

#[cfg(test)]
mod tests {}
