use reqwest::{Client, Proxy};
use std::time::Duration;

pub fn build_client_with_timeout(
    proxy: Option<&str>,
    timeout_secs: u64,
) -> Result<Client, reqwest::Error> {
    let mut headers = reqwest::header::HeaderMap::new();
    headers.insert(
        reqwest::header::USER_AGENT,
        reqwest::header::HeaderValue::from_static(
            "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 \
             (KHTML, like Gecko) Chrome/124.0.0.0 Safari/537.36",
        ),
    );
    headers.insert(
        reqwest::header::ACCEPT,
        reqwest::header::HeaderValue::from_static(
            "text/html,application/xhtml+xml,application/xml;q=0.9,image/avif,image/webp,*/*;q=0.8",
        ),
    );
    headers.insert(
        reqwest::header::ACCEPT_LANGUAGE,
        reqwest::header::HeaderValue::from_static("en-US,en;q=0.9"),
    );

    let mut client_builder = Client::builder()
        .timeout(Duration::from_secs(timeout_secs.clamp(1, 300)))
        .default_headers(headers)
        .brotli(true)
        .gzip(true);

    if let Some(proxy_url) = proxy {
        client_builder = client_builder.proxy(Proxy::all(proxy_url)?);
    }

    client_builder.build()
}
