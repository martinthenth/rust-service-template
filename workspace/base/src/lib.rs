pub mod config;
pub mod database;
pub mod error;
pub mod events;
pub mod outbox;
mod test;
pub mod users;

use crate::database::DbExecutor;

#[allow(async_fn_in_trait)]
pub trait Factory {
    #[cfg(feature = "testing")]
    fn factory() -> Self;

    #[cfg(feature = "testing")]
    async fn insert(&self, db: impl DbExecutor<'_>) -> Self;
}
