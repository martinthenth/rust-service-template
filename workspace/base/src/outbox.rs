use sea_query::Iden;
use sqlx::prelude::FromRow;
use time::OffsetDateTime;
use uuid::Uuid;

use crate::outbox_topic::OutboxTopic;

#[derive(Debug, FromRow, PartialEq)]
pub struct Outbox {
    pub id: Uuid,
    pub topic: OutboxTopic,
    pub key: Uuid,
    pub payload: Vec<u8>,
    pub timestamp: OffsetDateTime,
}

pub enum OutboxTable {
    Table,
    Id,
    Topic,
    Key,
    Payload,
    Timestamp,
}

impl Iden for OutboxTable {
    fn unquoted(&self, s: &mut dyn std::fmt::Write) {
        write!(
            s,
            "{}",
            match self {
                Self::Table => "outbox",
                Self::Id => "id",
                Self::Topic => "topic",
                Self::Key => "key",
                Self::Payload => "payload",
                Self::Timestamp => "timestamp",
            }
        )
        .unwrap();
    }
}
