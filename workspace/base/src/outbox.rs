use sea_query::Iden;
use sqlx::prelude::FromRow;
use time::OffsetDateTime;
use uuid::Uuid;

use crate::outbox_type::OutboxType;

#[derive(Debug, FromRow, PartialEq)]
pub struct Outbox {
    pub id: Uuid,
    pub r#type: OutboxType,
    pub payload: Vec<u8>,
    pub timestamp: OffsetDateTime,
}

pub enum OutboxTable {
    Table,
    Id,
    Type,
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
                Self::Type => "type",
                Self::Payload => "payload",
                Self::Timestamp => "timestamp",
            }
        )
        .unwrap();
    }
}
