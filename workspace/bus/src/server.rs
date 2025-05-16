use sea_streamer::Consumer;
use sea_streamer::ConsumerGroup;
use sea_streamer::ConsumerMode;
use sea_streamer::ConsumerOptions;
use sea_streamer::Message;
use sea_streamer::StreamKey;
use sea_streamer::Streamer;
use sea_streamer::kafka::AutoOffsetReset;
use sea_streamer::kafka::KafkaConsumerOptions;
use sea_streamer::kafka::KafkaStreamer;
use sqlx::PgPool;
use tracing::info;

use base::config::CONFIG;
use base::error::Error;
use tracing::warn;

use crate::handlers::users_events_handler::UsersEventsHandler;

#[derive(Debug)]
pub struct Server {}

impl Server {
    /// Start the server.
    pub async fn start_server(pool: PgPool) -> Result<(), Error> {
        let address = CONFIG
            .kafka_url
            .parse()
            .map_err(|e| Error::InternalServer(format!("Failed to parse Kafka URL: {e}")))?;
        let topic = StreamKey::new(CONFIG.kafka_topic.clone())
            .map_err(|_| Error::InternalServer("Failed to create stream key".to_string()))?;
        let group = ConsumerGroup::new(CONFIG.kafka_group.clone());
        let streamer = KafkaStreamer::connect(address, Default::default())
            .await
            .map_err(|e| Error::InternalServer(format!("Failed to connect to Kafka: {e}")))?;
        let mut options = KafkaConsumerOptions::new(ConsumerMode::LoadBalanced);
        options.set_auto_offset_reset(AutoOffsetReset::Earliest);
        options.set_enable_auto_commit(false);
        options.set_group_id(group);
        let consumer = streamer
            .create_consumer(&[topic], options)
            .await
            .map_err(|e| Error::InternalServer(format!("Failed to create consumer: {e}")))?;

        info!("Starting Bus Server listening to {}", CONFIG.kafka_topic);

        loop {
            let message = consumer
                .next()
                .await
                .map_err(|_| Error::InternalServer("Failed to get message".to_string()))?;

            match message.stream_key() {
                key if key.to_string().ends_with(".users.events") => {
                    UsersEventsHandler::handle_message(&pool, message).await?;
                }
                _ => {
                    warn!("Unhandled topic: {}", message.stream_key());
                }
            }
        }
    }
}
