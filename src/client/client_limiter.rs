use log::{debug, info, warn};
use std::collections::VecDeque;
use std::sync::Arc;
use std::time::{Duration, Instant};
use tokio::sync::Mutex;

/// Rate limiter for managing API request limits.
#[derive(Debug, Clone)]
pub struct RateLimiter {
    embeddings_limiter: Arc<Mutex<ApiLimiter>>,
    reranking_limiter: Arc<Mutex<ApiLimiter>>,
}

impl RateLimiter {
}

/// Internal structure for managing rate limits for a specific API.
#[derive(Debug)]
struct ApiLimiter {
    requests: VecDeque<Instant>,
    tokens: VecDeque<(Instant, u32)>,
    rpm_limit: u32,
    tpm_limit: u32,
}

impl RateLimiter {
    /// Creates a new `RateLimiter` instance with default limits.
    pub fn new() -> Self {
        debug!("Creating new RateLimiter");
        Self {
            embeddings_limiter: Arc::new(Mutex::new(ApiLimiter::new(300, 1_000_000))),
            reranking_limiter: Arc::new(Mutex::new(ApiLimiter::new(100, 2_000_000))),
        }
    }

    /// Checks if the embeddings API limit has been reached.
    ///
    /// # Arguments
    ///
    /// * `tokens` - The number of tokens in the request.
    ///
    /// # Returns
    ///
    /// The duration to wait before making the request.
    pub async fn check_embeddings_limit(&self, tokens: u32) -> Duration {
        debug!("Checking embeddings limit for {} tokens", tokens);
        self.embeddings_limiter.lock().await.check_limit(tokens)
    }

    /// Updates the usage for the embeddings API.
    ///
    /// # Arguments
    ///
    /// * `tokens` - The number of tokens used in the request.
    pub async fn update_embeddings_usage(&self, tokens: u32) {
        debug!("Updating embeddings usage with {} tokens", tokens);
        self.embeddings_limiter.lock().await.update_usage(tokens);
    }

    /// Checks if the reranking API limit has been reached.
    ///
    /// # Arguments
    ///
    /// * `tokens` - The number of tokens in the request.
    ///
    /// # Returns
    ///
    /// The duration to wait before making the request.
    pub async fn check_reranking_limit(&self, tokens: u32) -> Duration {
        debug!("Checking reranking limit for {} tokens", tokens);
        self.reranking_limiter.lock().await.check_limit(tokens)
    }

    /// Updates the usage for the reranking API.
    ///
    /// # Arguments
    ///
    /// * `tokens` - The number of tokens used in the request.
    pub async fn update_reranking_usage(&self, tokens: u32) {
        debug!("Updating reranking usage with {} tokens", tokens);
        self.reranking_limiter.lock().await.update_usage(tokens);
    }
}

impl ApiLimiter {
    /// Creates a new `ApiLimiter` instance.
    ///
    /// # Arguments
    ///
    /// * `rpm_limit` - The requests per minute limit.
    /// * `tpm_limit` - The tokens per minute limit.
    fn new(rpm_limit: u32, tpm_limit: u32) -> Self {
        debug!(
            "Creating new ApiLimiter with RPM: {}, TPM: {}",
            rpm_limit, tpm_limit
        );
        Self {
            requests: VecDeque::new(),
            tokens: VecDeque::new(),
            rpm_limit,
            tpm_limit,
        }
    }

    /// Checks if the API limit has been reached.
    ///
    /// # Arguments
    ///
    /// * `tokens` - The number of tokens in the request.
    ///
    /// # Returns
    ///
    /// The duration to wait before making the request.
    fn check_limit(&mut self, tokens: u32) -> Duration {
        let now = Instant::now();
        self.clean_old_entries(now);

        let requests_wait = self.check_rpm_limit(now);
        let tokens_wait = self.check_tpm_limit(now, tokens);

        let wait_time = requests_wait.max(tokens_wait);
        if !wait_time.is_zero() {
            info!("Rate limit reached. Wait time: {:?}", wait_time);
        }
        wait_time
    }

    /// Updates the usage for the API.
    ///
    /// # Arguments
    ///
    /// * `tokens` - The number of tokens used in the request.
    fn update_usage(&mut self, tokens: u32) {
        let now = Instant::now();
        self.requests.push_back(now);
        self.tokens.push_back((now, tokens));
        debug!(
            "Updated usage. Total requests: {}, Total tokens: {}",
            self.requests.len(),
            self.tokens.iter().map(|&(_, t)| t).sum::<u32>()
        );
    }

    /// Removes entries older than one minute.
    ///
    /// # Arguments
    ///
    /// * `now` - The current time.
    fn clean_old_entries(&mut self, now: Instant) {
        let one_minute_ago = now - Duration::from_secs(60);
        let old_requests = self.requests.len();
        let old_tokens = self.tokens.len();
        self.requests.retain(|&time| time > one_minute_ago);
        self.tokens.retain(|&(time, _)| time > one_minute_ago);
        debug!(
            "Cleaned old entries. Removed requests: {}, Removed token entries: {}",
            old_requests - self.requests.len(),
            old_tokens - self.tokens.len()
        );
    }

    /// Checks if the requests per minute limit has been reached.
    ///
    /// # Arguments
    ///
    /// * `now` - The current time.
    ///
    /// # Returns
    ///
    /// The duration to wait before making the request.
    fn check_rpm_limit(&self, now: Instant) -> Duration {
        if self.requests.len() as u32 >= self.rpm_limit {
            if let Some(&oldest) = self.requests.front() {
                let wait_time = oldest + Duration::from_secs(60) - now;
                if wait_time.as_secs() > 0 {
                    warn!("RPM limit reached. Wait time: {:?}", wait_time);
                    return wait_time;
                }
            }
        }
        Duration::from_secs(0)
    }

    /// Checks if the tokens per minute limit has been reached.
    ///
    /// # Arguments
    ///
    /// * `now` - The current time.
    /// * `new_tokens` - The number of tokens in the new request.
    ///
    /// # Returns
    ///
    /// The duration to wait before making the request.
    fn check_tpm_limit(&self, now: Instant, new_tokens: u32) -> Duration {
        let current_tokens: u32 = self.tokens.iter().map(|&(_, tokens)| tokens).sum();
        if current_tokens + new_tokens > self.tpm_limit {
            if let Some(&(oldest, _)) = self.tokens.front() {
                let wait_time = oldest + Duration::from_secs(60) - now;
                if wait_time.as_secs() > 0 {
                    warn!(
                        "TPM limit reached. Current tokens: {}, New tokens: {}, Wait time: {:?}",
                        current_tokens, new_tokens, wait_time
                    );
                    return wait_time;
                }
            }
        }
        Duration::from_secs(0)
    }
}

impl Default for RateLimiter {
    fn default() -> Self {
        Self::new()
    }
}
