use sea_streamer::Buffer;
use sea_streamer::ConnectOptions;
use sea_streamer::Consumer;
use sea_streamer::ConsumerGroup;
use sea_streamer::ConsumerMode;
use sea_streamer::ConsumerOptions;
use sea_streamer::Message;
use sea_streamer::StreamKey;
use sea_streamer::StreamUrl;
use sea_streamer::Streamer;
use sea_streamer::export::futures::StreamExt;
use sea_streamer::kafka::AutoOffsetReset;
use sea_streamer::kafka::KafkaConsumer;
use sea_streamer::kafka::KafkaConsumerOptions;
use sea_streamer::kafka::KafkaMessage;
use sea_streamer::kafka::KafkaStreamer;
use sqlx::PgPool;
use tracing::info;

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
            .map_err(|e| Error::InternalServer(format!("Failed to parse RPC URL: {e}")))?;
        // let address = StreamUrl::from(kafka_url);
        let streamer = KafkaStreamer::connect(address, Default::default())
            .await
            .map_err(|e| Error::InternalServer(format!("Failed to connect to Kafka: {e}")))?;
        // TODO: Get topic from config
        let topic = StreamKey::new("dev.users.events")
            .map_err(|_| Error::InternalServer("Failed to create stream key".to_string()))?;
        let mut options = KafkaConsumerOptions::new(ConsumerMode::LoadBalanced);
        options.set_auto_offset_reset(AutoOffsetReset::Earliest);
        options.set_enable_auto_commit(false);
        // TODO: Get group from config
        options.set_group_id(ConsumerGroup::new("some-group"));
        let mut consumer = streamer
            .create_consumer(&[topic], options)
            .await
            .map_err(|e| Error::InternalServer(format!("Failed to create consumer: {e}")))?;

        let messages = Self::consume(&mut consumer, 10).await;

        println!("messages: {:?}", messages);

        // loop {
        //     let mess: KafkaMessage = consumer.next().await?;
        //     println!("[{}] {}", mess.timestamp(), mess.message().as_str()?);
        // }

        info!("Starting Bus Server");

        Ok(())
    }

    async fn consume(consumer: &mut KafkaConsumer, num: usize) -> Vec<usize> {
        consumer
            .stream()
            .take(num)
            .map(|mess| {
                mess.unwrap()
                    .message()
                    .as_str()
                    .unwrap()
                    .parse::<usize>()
                    .unwrap()
            })
            .collect::<Vec<usize>>()
            .await
    }
}
