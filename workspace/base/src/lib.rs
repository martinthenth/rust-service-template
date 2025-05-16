pub mod config;
pub mod database;
pub mod error;
mod event;
mod event_domain;
mod event_type;
mod events;
mod outbox;
mod outbox_domain;
mod outbox_kind;
mod outbox_type;
mod outboxes;
mod test;
pub mod user;
mod user_events;
pub mod users;

use crate::database::DbExecutor;

#[allow(async_fn_in_trait)]
pub trait Factory {
    #[cfg(feature = "testing")]
    fn factory() -> Self;

    #[cfg(feature = "testing")]
    async fn insert(db: impl DbExecutor<'_>, s: Self) -> Self;
}
