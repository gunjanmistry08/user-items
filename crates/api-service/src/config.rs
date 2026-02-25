use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
pub struct Config {
    pub server_host: String,
    pub server_port: u16,
    pub kafka_brokers: String,
    pub kafka_topic: String,
}

impl Config {
    pub fn from_env() -> Self {
        envy::from_env::<Config>().expect("Failed to load configuration from environment")
    }
}
