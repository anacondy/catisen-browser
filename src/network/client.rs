use reqwest::{Client, Proxy};
use std::time::Duration;

pub fn build_client(proxy: Option<&str>) -> Result<Client, reqwest::Error> {
    let mut headers = reqwest::header::HeaderMap::new();
    // Stealth headers to bypass basic bot detectors
    headers.insert(reqwest::header::USER_AGENT, reqwest::header::HeaderValue::from_static("Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/124.0.0.0 Safari/537.36"));
    headers.insert(reqwest::header::ACCEPT, reqwest::header::HeaderValue::from_static("text/html,application/xhtml+xml,application/xml;q=0.9,image/avif,image/webp,image/apng,*/*;q=0.8,application/signed-exchange;v=b3;q=0.7"));
    headers.insert(reqwest::header::ACCEPT_LANGUAGE, reqwest::header::HeaderValue::from_static("en-US,en;q=0.9"));
    headers.insert("Sec-Ch-Ua", reqwest::header::HeaderValue::from_static("\"Chromium\";v=\"124\", \"Google Chrome\";v=\"124\", \"Not-A.Brand\";v=\"99\""));
    headers.insert("Sec-Ch-Ua-Mobile", reqwest::header::HeaderValue::from_static("?0"));
    headers.insert("Sec-Ch-Ua-Platform", reqwest::header::HeaderValue::from_static("\"Windows\""));
    headers.insert("Sec-Fetch-Dest", reqwest::header::HeaderValue::from_static("document"));
    headers.insert("Sec-Fetch-Mode", reqwest::header::HeaderValue::from_static("navigate"));
    headers.insert("Sec-Fetch-Site", reqwest::header::HeaderValue::from_static("none"));
    headers.insert("Sec-Fetch-User", reqwest::header::HeaderValue::from_static("?1"));
    headers.insert("Upgrade-Insecure-Requests", reqwest::header::HeaderValue::from_static("1"));

    let mut client_builder = Client::builder()
        .timeout(Duration::from_secs(30))
        .default_headers(headers)
        .brotli(true)
        .gzip(true);

    if let Some(proxy_url) = proxy {
        client_builder = client_builder.proxy(Proxy::all(proxy_url)?);
    }

    client_builder.build()
}
