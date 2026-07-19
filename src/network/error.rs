use thiserror::Error;

#[derive(Debug, Error)]
pub enum FetchError {
    #[error("Tor proxy is not reachable at {proxy} ({route_source}).")]
    TorProxyUnavailable {
        proxy: String,
        route_source: String,
    },

    #[error("Invalid Tor proxy configuration: {0}")]
    InvalidTorProxy(String),

    #[error("Failed to initialize HTTP client: {0}")]
    ClientInit(String),

    #[error("Request timed out while loading {url} (attempt {attempt}/{max_attempts}).")]
    Timeout {
        url: String,
        attempt: u8,
        max_attempts: u8,
        use_tor: bool,
    },

    #[error("Network request failed for {url} on attempt {attempt}/{max_attempts}: {source}")]
    Request {
        url: String,
        attempt: u8,
        max_attempts: u8,
        use_tor: bool,
        #[source]
        source: reqwest::Error,
    },

    #[error("Failed to stream response body from {url}: {source}")]
    Stream {
        url: String,
        #[source]
        source: reqwest::Error,
    },

    #[error("HTTP request failed: {0}")]
    Reqwest(#[from] reqwest::Error),
}

impl FetchError {
    pub fn is_retryable(&self) -> bool {
        match self {
            FetchError::Timeout { .. } => true,
            FetchError::Request { source, .. } => {
                source.is_timeout() || source.is_connect() || source.is_request()
            }
            _ => false,
        }
    }

    pub fn user_message(&self) -> String {
        match self {
            FetchError::TorProxyUnavailable { proxy, .. } => format!(
                "❌ Tor proxy is unreachable at {}.\n\nAction: start Tor Browser/Tor daemon, or set CATISEN_TOR_PROXY to a reachable SOCKS endpoint.",
                proxy
            ),
            FetchError::InvalidTorProxy(value) => format!(
                "❌ Invalid Tor proxy value: {}\n\nAction: use a valid URI like socks5h://127.0.0.1:9150.",
                value
            ),
            FetchError::ClientInit(msg) => format!(
                "❌ Failed to initialize network client.\n\nDetails: {}",
                msg
            ),
            FetchError::Timeout {
                url,
                attempt,
                max_attempts,
                use_tor,
            } => {
                let route_hint = if *use_tor {
                    "Tor route"
                } else {
                    "direct clearnet route"
                };
                format!(
                    "❌ Request timed out for {} (attempt {}/{} via {}).\n\nAction: retry, or increase timeout in settings.",
                    url, attempt, max_attempts, route_hint
                )
            }
            FetchError::Request {
                url,
                attempt,
                max_attempts,
                source,
                use_tor,
            } => {
                let route_hint = if *use_tor {
                    "Tor"
                } else {
                    "direct"
                };
                format!(
                    "❌ Network request failed for {} (attempt {}/{} via {}).\n\nDetails: {}",
                    url, attempt, max_attempts, route_hint, source
                )
            }
            FetchError::Stream { url, source } => format!(
                "❌ Failed to read response stream from {}.\n\nDetails: {}",
                url, source
            ),
            FetchError::Reqwest(_) => "❌ HTTP request failed.".to_string(),
        }
    }
}
