use lapin::{Connection, ConnectionProperties, Consumer};
use lapin::options::{BasicAckOptions, BasicConsumeOptions};
use lapin::types::FieldTable;
use crate::config::{Rabbitmq, Settings};
// use lapin::Consumer::next;
use futures_lite::stream::StreamExt;
struct RabbitmqConnection {
    conn: Connection,
    config: Rabbitmq,
    consumer: Consumer,
}

impl RabbitmqConnection {
    async fn new(config: Rabbitmq) -> RabbitmqConnection {
        let conn = Connection::connect(
            &config.uri,
            ConnectionProperties::default(),
        ).await.unwrap_or_else(|e| {
            panic!("failed to connect to rabbitmq: {:?}", e);
        });

        let channel = conn.create_channel().await.unwrap_or_else(|e| {
            panic!("failed to create channel");
        });
        let mut consumer = channel
            .basic_consume(
                config.queue.as_str(),
                config.tag.as_str(),
                BasicConsumeOptions::default(),
                FieldTable::default(),
            )
            .await.unwrap_or_else(|e| {
            panic!("failed to create consumer");
        });

        RabbitmqConnection {
            conn,
            config,
            consumer,
        }
    }

    async fn process_queue(&mut self) {
        while let Some(delivery) = self.consumer.next().await {
            let delivery = delivery.expect("error in consumer");
            delivery
                .ack(BasicAckOptions::default())
                .await
                .expect("ack");
        }
    }
}

#[cfg(test)]
mod rabbitmq {
    use crate::config::{Rabbitmq, Settings};
    use crate::rabbitmq_connection::RabbitmqConnection;

    #[tokio::test]
    async fn test_connection() {
        let config = Settings::new().expect("config can be loaded");
        let t = RabbitmqConnection::new(config.rabbitmq).await;
    }
}