use base64::prelude::*;
use prost::Message as ProstMessage;
use sea_streamer::Buffer;
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
use serde_json::Value;
use sqlx::PgPool;
use tracing::info;
use tracing::warn;

use crate::common::Envelope;
use crate::handlers::users_events_handler::UsersEventsHandler;
use base::config::CONFIG;
use base::error::Error;

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
            .map_err(|e| Error::InternalServer(format!("Failed to create stream key: {e}")))?;
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
                .map_err(|e| Error::InternalServer(format!("Failed to get message: {e}")))?;
            let topic = message.stream_key();
            let payload = message.message();
            // TODO: Handle error for unwrap
            let json: Value = serde_json::from_str(
                payload
                    .as_str()
                    .map_err(|e| Error::InternalServer(format!("Failed to get payload: {e}")))?,
            )
            .map_err(|e| Error::InternalServer(format!("Failed to parse JSON: {}", e)))?;
            let payload = json["payload"]
                .as_str()
                .ok_or_else(|| Error::InternalServer("Missing payload field".to_string()))?;
            let message_bytes = BASE64_STANDARD
                .decode(payload)
                .map_err(|e| Error::InternalServer(format!("Failed to decode base64: {}", e)))?;
            let envelope = Envelope::decode(message_bytes.as_slice())
                .map_err(|e| Error::InternalServer(format!("Failed to decode message: {}", e)))?;

            match topic {
                key if key.to_string().ends_with(".users.events") => {
                    UsersEventsHandler::handle_message(&pool, envelope).await?
                }
                _ => warn!("Unhandled topic: {}", topic),
            }
        }
    }
}
