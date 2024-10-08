use std::sync::Arc;
use std::time::{Duration, Instant};
use tokio::sync::Mutex;

#[derive(Debug, Clone)]
pub struct RateLimiter {
    last_request_time: Arc<Mutex<Instant>>,
    pub rate_limit_duration: Duration,
}

impl RateLimiter {
    pub fn new(rate_limit_duration: Duration) -> Self {
        Self {
            last_request_time: Arc::new(Mutex::new(Instant::now())),
            rate_limit_duration,
        }
    }

    pub async fn wait(&self) {
        let mut last_request = self.last_request_time.lock().await;
        let now = Instant::now();

        if now.duration_since(*last_request) < self.rate_limit_duration {
            let sleep_duration = self.rate_limit_duration - now.duration_since(*last_request);
            tokio::time::sleep(sleep_duration).await;
        }

        *last_request = Instant::now();
    }
}
