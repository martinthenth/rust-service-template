use sea_streamer::Buffer;
use sea_streamer::Message;
use sea_streamer::kafka::KafkaMessage;
use sqlx::PgPool;
use tracing::info;

use base::error::Error;

pub struct UsersEventsHandler;

impl UsersEventsHandler {
    pub async fn handle_message(pool: &PgPool, message: KafkaMessage<'_>) -> Result<(), Error> {
        info!(
            "{} {} {}, {}",
            message.stream_key(),
            message.shard_id(),
            message.timestamp(),
            message.message().as_str().unwrap()
        );

        Ok(())
    }
}
