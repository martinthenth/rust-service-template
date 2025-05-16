use base::database::DbExecutor;
use base64::prelude::*;
use prost::Message as ProstMessage;
use sea_streamer::Buffer;
use sea_streamer::Message;
use sea_streamer::kafka::KafkaMessage;
use serde_json::Value;
use sqlx::PgPool;
use tracing::info;
use tracing::instrument;

use crate::users::events::UserCreated;
use base::error::Error;

pub struct UsersEventsHandler;

impl UsersEventsHandler {
    #[instrument]
    pub async fn handle_message(
        pool: impl DbExecutor<'_>,
        message: KafkaMessage<'_>,
    ) -> Result<(), Error> {
        let content = message.message();
        let json: Value = serde_json::from_str(content.as_str().unwrap())
            .map_err(|e| Error::InternalServer(format!("Failed to parse JSON: {}", e)))?;
        let payload = json["payload"]
            .as_str()
            .ok_or_else(|| Error::InternalServer("Missing payload field".to_string()))?;
        let message_bytes = BASE64_STANDARD
            .decode(payload)
            .map_err(|e| Error::InternalServer(format!("Failed to decode base64: {}", e)))?;
        let user_created = UserCreated::decode(message_bytes.as_slice())
            .map_err(|e| Error::InternalServer(format!("Failed to decode message: {}", e)))?;

        println!("user_created: {:?}", user_created);

        Ok(())
    }
}
