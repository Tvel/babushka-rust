extern crate serde_json;
extern crate reqwest;

pub async fn get_json(url: &str) -> ::reqwest::Result<serde_json::Value> {
    get_from_web(url).await?.json().await
}

pub async fn get_plain(url: &str) -> ::reqwest::Result<String> {
    get_from_web(url).await?.text().await
}

async fn get_from_web(url: &str) -> ::reqwest::Result<reqwest::Response> {
    reqwest::Client::new()
        .get(url)
        .send()
        .await
}

pub async fn get_plain2(url: &str) -> ::reqwest::Result<String> {
    reqwest::Client::new()
        .get(url)
        .header("Accept", "text/plain")
        .send()
        .await?
        .text()
        .await
}