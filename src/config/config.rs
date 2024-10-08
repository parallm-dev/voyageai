use serde::Deserialize;
use std::time::Duration;

#[derive(Debug, Clone, Deserialize)]
pub struct VoyageConfig {
    pub api_key: String,
    pub model: String,
    pub base_url: String,
    pub timeout: u64,
    pub rate_limit_duration: Duration,
    pub default_embedding_model: String,
}

impl VoyageConfig {
    pub fn new(api_key: String, model: String) -> Self {
        VoyageConfig {
            api_key,
            model,
            base_url: String::from("https://api.voyageai.com/v1"),
            timeout: 30,
            rate_limit_duration: Duration::from_millis(100),
            default_embedding_model: String::from("voyage-3"),
        }
    }

    // ... existing methods ...
}
