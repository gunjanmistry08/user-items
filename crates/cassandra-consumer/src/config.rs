use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
pub struct Config {
    pub kafka_brokers: String,
    pub kafka_topic: String,
    pub kafka_group_id: String,
    pub cassandra_uri: String,
}

impl Config {
    pub fn from_env() -> Self {
        envy::from_env().expect("Failed to load config")
    }
}
