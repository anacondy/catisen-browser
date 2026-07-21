use reqwest::{header::HeaderMap, Client};
use super::error::FetchError;

pub async fn fetch_url(client: &Client, url: &str) -> Result<(HeaderMap, String), FetchError> {
    let res = client.get(url).send().await?;
    let headers = res.headers().clone();
    let text = res.text().await?;
    Ok((headers, text))
}
