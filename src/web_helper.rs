extern crate serde_json;
extern crate reqwest;

pub fn get_json(url: &str) -> ::reqwest::Result<serde_json::Value> {
    get_from_web(url)?.json()
}

pub fn get_plain(url: &str) -> ::reqwest::Result<String> {
    get_from_web(url)?.text()
}

fn get_from_web(url: &str) -> ::reqwest::Result<reqwest::Response> {
    reqwest::Client::new()
        .get(url)
        .send()
}