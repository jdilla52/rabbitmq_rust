use serde::Deserialize;
use std::fmt;
use ::config::{Config, ConfigError, File};

#[derive(Debug, Deserialize, Clone)]
pub struct Log {
    pub level: String,
}

#[derive(Debug, Deserialize, Clone)]
pub struct Rabbitmq {
    pub uri: String,
    pub queue: String,
    pub tag: String
}

#[derive(Debug, Deserialize, Clone)]
pub struct Settings {
    pub rabbitmq: Rabbitmq,
    pub log: Log,
    pub env: ENV,
}

const CONFIG_FILE_PREFIX: &str = "./config/";

impl Settings {
    pub fn new() -> Result<Self, ConfigError> {
        let env = std::env::var("RUN_ENV").unwrap_or_else(|_| "Development".into());
        let mut s = Config::new();
        s.set("env", env.clone())?;

        s.merge(File::with_name(&format!("{}{}", CONFIG_FILE_PREFIX, env)))?;

        s.try_into()
    }
}

#[derive(Clone, Debug, Deserialize)]
pub enum ENV {
    Development,
    Testing,
    Production,
}

impl fmt::Display for ENV {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ENV::Development => write!(f, "Development"),
            ENV::Testing => write!(f, "Testing"),
            ENV::Production => write!(f, "Production"),
        }
    }
}

impl From<&str> for ENV {
    fn from(env: &str) -> Self {
        match env {
            "Testing" => ENV::Testing,
            "Production" => ENV::Production,
            _ => ENV::Development,
        }
    }
}

#[cfg(test)]
mod config{
    use crate::config::Settings;
    #[test]
    fn from_file(){
        let settings = Settings::new().unwrap_or_else(|e|{
            panic!("failed to create env");
        });
    }
}