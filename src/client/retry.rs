use crate::errors::VoyageError;
use log::{debug, info, warn};
use std::time::Duration;
use tokio::time::sleep;

/// Retries an asynchronous operation with exponential backoff.
///
/// This function will retry the given operation up to `max_retries` times,
/// with an exponentially increasing delay between retries. It specifically
/// handles rate limit errors, waiting for the specified reset time before retrying.
///
/// # Arguments
///
/// * `operation` - A closure that returns a Future which resolves to a Result.
/// * `max_retries` - The maximum number of retry attempts.
/// * `initial_delay` - The initial delay duration before the first retry.
///
/// # Type Parameters
///
/// * `F` - The type of the closure that generates the Future.
/// * `Fut` - The type of the Future returned by the closure.
/// * `T` - The type of the successful result.
///
/// # Returns
///
/// Returns a Result containing either the successful result of type `T`,
/// or a `VoyageError` if all retry attempts fail.
///
/// # Examples
///
/// ```
/// use std::time::Duration;
/// use voyageai::client::retry::retry_with_exponential_backoff;
/// use voyageai::errors::VoyageError;
///
/// async fn fallible_operation() -> Result<String, VoyageError> {
///     // Simulated operation that might fail
///     Err(VoyageError::RateLimitExceeded { reset_in: Duration::from_secs(1) })
/// }
///
/// #[tokio::main]
/// async fn main() {
///     let result = retry_with_exponential_backoff(
///         || fallible_operation(),
///         3,
///         Duration::from_millis(100)
///     ).await;
///
///     match result {
///         Ok(value) => println!("Operation succeeded: {}", value),
///         Err(e) => println!("Operation failed after retries: {:?}", e),
///     }
/// }
/// ```
#[allow(dead_code)]
pub async fn retry_with_exponential_backoff<F, Fut, T>(
    mut operation: F,
    max_retries: u32,
    initial_delay: Duration,
) -> Result<T, VoyageError>
where
    F: FnMut() -> Fut,
    Fut: std::future::Future<Output = Result<T, VoyageError>>,
{
    let mut retries = 0;
    let mut delay = initial_delay;

    debug!(
        "Starting retry_with_exponential_backoff with max_retries: {}, initial_delay: {:?}",
        max_retries, initial_delay
    );

    loop {
        debug!("Attempting operation, retry count: {}", retries);
        match operation().await {
            Ok(result) => {
                info!("Operation succeeded after {} retries", retries);
                return Ok(result);
            }
            Err(VoyageError::RateLimitExceeded { reset_in }) => {
                if retries >= max_retries {
                    warn!(
                        "Max retries ({}) reached. Returning RateLimitExceeded error",
                        max_retries
                    );
                    return Err(VoyageError::RateLimitExceeded { reset_in });
                }
                info!(
                    "Rate limit exceeded. Waiting for {:?} before retry",
                    reset_in
                );
                sleep(reset_in).await;
                retries += 1;
                delay *= 2; // Exponential backoff
                debug!("Increased delay to {:?} for next retry", delay);
            }
            Err(e) => {
                warn!("Operation failed with error: {:?}", e);
                return Err(e);
            }
        }
    }
}
