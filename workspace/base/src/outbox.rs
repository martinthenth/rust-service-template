use time::OffsetDateTime;
use uuid::Uuid;

use crate::error::Error;

pub struct Message {
    pub id: Uuid,
    pub typx: String,
    pub payload: Vec<u8>,
    pub timestamp: OffsetDateTime,
}

pub struct Outbox {}

impl Outbox {
    /// Store a message.
    pub async fn store_message(message: Message) -> Result<Message, Error> {
        Ok(message)
    }
}

#[cfg(test)]
mod tests {}
