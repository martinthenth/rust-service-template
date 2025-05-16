use sea_query::Iden;
use sqlx::prelude::FromRow;
use time::OffsetDateTime;
use uuid::Uuid;

use crate::outbox_domain::OutboxDomain;
use crate::outbox_kind::OutboxKind;
use crate::outbox_type::OutboxType;

#[derive(Debug, FromRow, PartialEq)]
pub struct Outbox {
    pub id: Uuid,
    pub domain: OutboxDomain,
    pub kind: OutboxKind,
    pub r#type: OutboxType,
    pub key: Uuid,
    pub payload: Vec<u8>,
    pub timestamp: OffsetDateTime,
}

pub enum OutboxTable {
    Table,
    Id,
    Domain,
    Kind,
    Type,
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
                Self::Domain => "domain",
                Self::Kind => "kind",
                Self::Type => "type",
                Self::Key => "key",
                Self::Payload => "payload",
                Self::Timestamp => "timestamp",
            }
        )
        .unwrap();
    }
}
