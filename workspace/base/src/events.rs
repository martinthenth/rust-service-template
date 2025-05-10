use time::OffsetDateTime;
use uuid::Uuid;

use crate::error::Error;

pub struct Event {
    pub id: Uuid,
    pub typx: String,
    pub payload: Vec<u8>,
    pub timestamp: OffsetDateTime,
}

pub struct Events {}

impl Events {
    /// Check if an event exists by its ID.
    pub async fn check_event_by_id(_id: Uuid) -> Result<bool, Error> {
        Ok(true)
    }

    /// Store an event.
    pub async fn store_event(event: Event) -> Result<Event, Error> {
        Ok(event)
    }
}

#[cfg(test)]
mod tests {}
