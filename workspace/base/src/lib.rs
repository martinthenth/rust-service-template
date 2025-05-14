pub mod config;
pub mod database;
pub mod error;
mod messages;
mod outbox;
mod outbox_type;
mod outboxes;
mod test;
pub mod user_events;
pub mod users;

use crate::database::DbExecutor;

#[allow(async_fn_in_trait)]
pub trait Factory {
    #[cfg(feature = "testing")]
    fn factory() -> Self;

    #[cfg(feature = "testing")]
    async fn insert(db: impl DbExecutor<'_>, s: Self) -> Self;
}
