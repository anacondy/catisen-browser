use reqwest::{header::HeaderMap, Client};
use std::time::Duration;

use super::error::FetchError;

/// Fetch a URL with bounded retries and return the real HTTP status code.
///
/// The caller supplies the selected route so error messages can distinguish a
/// direct request from a Tor request. HTTP error statuses are returned as data;
/// transport and body-read failures use the typed `FetchError` variants.
pub async fn fetch_url(
    client: &Client,
    url: &str,
    max_attempts: u8,
    use_tor: bool,
) -> Result<(u16, HeaderMap, String), FetchError> {
    let max_attempts = max_attempts.clamp(1, 5);
    let mut last_error = None;

    for attempt in 1..=max_attempts {
        match client.get(url).send().await {
            Ok(response) => {
                let status = response.status().as_u16();
                let headers = response.headers().clone();
                match response.text().await {
                    Ok(body) => return Ok((status, headers, body)),
                    Err(source) => {
                        let error = FetchError::Stream {
                            url: url.to_string(),
                            source,
                        };
                        if attempt == max_attempts {
                            return Err(error);
                        }
                        last_error = Some(error);
                    }
                }
            }
            Err(source) => {
                let error = if source.is_timeout() {
                    FetchError::Timeout {
                        url: url.to_string(),
                        attempt,
                        max_attempts,
                        use_tor,
                    }
                } else {
                    FetchError::Request {
                        url: url.to_string(),
                        attempt,
                        max_attempts,
                        use_tor,
                        source,
                    }
                };
                let retryable = error.is_retryable();
                if !retryable || attempt == max_attempts {
                    return Err(error);
                }
                last_error = Some(error);
            }
        }

        // Keep retries bounded and avoid a tight loop against a dead proxy.
        tokio::time::sleep(Duration::from_millis(250 * u64::from(attempt))).await;
    }

    Err(last_error.unwrap_or_else(|| {
        FetchError::ClientInit("fetch_url: no attempts were made".to_string())
    }))
}
