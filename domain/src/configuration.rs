use std::default;

use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct Configuration {
    pub kafka: KafkaConfiguration,
}

impl Configuration {
    pub fn new(kafka: KafkaConfiguration) -> Self {
        Self { kafka }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct KafkaConfiguration {
    pub brokers: String,
    pub group_id: String,
    pub topic: String,
}

impl KafkaConfiguration {
    pub fn new(brokers: String, group_id: String, topic: String) -> Self {
        Self {
            brokers,
            group_id,
            topic,
        }
    }
}

impl Default for KafkaConfiguration {
    fn default() -> Self {
        Self {
            brokers: "localhost:9092".to_string(),
            group_id: "my-group".to_string(),
            topic: "my-topic".to_string(),
        }
    }
}
