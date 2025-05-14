use sqlx::prelude::FromRow;
use time::OffsetDateTime;
use uuid::Uuid;

#[derive(Debug, FromRow, PartialEq)]
pub struct Message {
    pub id: Uuid,
    pub typx: String,
    pub payload: Vec<u8>,
    pub timestamp: OffsetDateTime,
}
