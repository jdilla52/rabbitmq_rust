use lapin::{Connection, ConnectionProperties};
use lazy_static::lazy_static;
use crate::config::Settings;
lazy_static! {
    static ref CONFIG: rabbitmq_rust::config::Settings =
        rabbitmq_rust::config::Settings::new().expect("config can be loaded");
}

struct AppState {
    config: CONFIG,
    rabbitmq: Connection,
}

impl AppState {
    async fn new() {
        let settings = Settings::new().expect("config can be loaded");
        let conn = Connection::connect(
            &settings.rabbitmq.uri,
            ConnectionProperties::default(),
        ).await.unwrap_or_else(|e| {
            panic!("failed to connect: {:?}", e);
        });
    }
}