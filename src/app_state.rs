use lapin::{Connection, ConnectionProperties};
use lazy_static::lazy_static;
use crate::config::Settings;
lazy_static! {
    static ref CONFIG: Settings =
        Settings::new().expect("config can be loaded");
}

struct AppState {
    config: Settings,
}

impl AppState {
    async fn new()->AppState {
        let config = Settings::new().expect("config can be loaded");
        AppState{
            config
        }
    }
}