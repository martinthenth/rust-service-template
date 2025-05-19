pub mod config;
pub mod database;
pub mod error;
pub mod event;
pub mod event_type;
pub mod events;
mod outbox;
mod outbox_topic;
mod outbox_type;
mod outboxes;
mod test;
pub mod user;
mod user_outboxes;
pub mod users;

use crate::database::DbExecutor;

pub mod common {
    include!(concat!(env!("OUT_DIR"), "/example.common.v1.rs"));
}

#[allow(async_fn_in_trait)]
pub trait Factory {
    #[cfg(feature = "testing")]
    fn factory() -> Self;

    #[cfg(feature = "testing")]
    async fn insert(db: impl DbExecutor<'_>, s: Self) -> Self;
}
