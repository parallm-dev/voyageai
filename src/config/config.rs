use serde::Deserialize;
use std::time::Duration;

#[derive(Debug, Clone, Deserialize, Default)]
pub struct VoyageConfig {
    #[serde(default)]
    pub api_key: String,
    #[serde(default)]
    pub model: String,
    #[serde(default = "default_base_url")]
    pub base_url: String,
    #[serde(default = "default_timeout")]
    pub timeout: u64,
    #[serde(default = "default_rate_limit_duration")]
    pub rate_limit_duration: Duration,
    #[serde(default = "default_embedding_model")]
    pub default_embedding_model: String,
}

impl VoyageConfig {
    pub fn new(api_key: String, model: String) -> Self {
        VoyageConfig {
            api_key,
            model,
            base_url: default_base_url(),
            timeout: default_timeout(),
            rate_limit_duration: default_rate_limit_duration(),
            default_embedding_model: default_embedding_model(),
        }
    }
}

fn default_base_url() -> String {
    String::from("https://api.voyageai.com/v1")
}

fn default_timeout() -> u64 {
    30
}

fn default_rate_limit_duration() -> Duration {
    Duration::from_millis(100)
}

fn default_embedding_model() -> String {
    String::from("voyage-3")
}
