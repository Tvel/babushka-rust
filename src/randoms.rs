extern crate serde_json;
extern crate reqwest;

pub fn woof_image() -> Result<String, String> {
    let response = reqwest::Client::new()
        .get("https://random.dog/woof")
        .send()
        .map_err(|_| "Error getting a doggo right now :(")?
        .text()
        .map_err(|_| "Error getting a doggo right now :(")?;

    let url = format!("https://random.dog/{}", response);
    Ok(url)
}

pub fn meow_image() -> Result<String, String> {
    let response: serde_json::Value = reqwest::Client::new()
        .get("http://aws.random.cat/meow")
        .send()
        .map_err(|_| "Error getting a kitteh right now :(")?
        .json()
        .map_err(|_| "Error getting a kitteh right now :(")?;

    Ok(String::from(response["file"].as_str().unwrap()))
}

pub fn duck_image() -> Result<String, String> {
    let response: serde_json::Value = reqwest::Client::new().
        get("https://random-d.uk/api/v1/random")
        .send()
        .map_err(|_| "Error getting a ducky right now :(")?
        .json()
        .map_err(|_| "Error getting a ducky right now :(")?;

    Ok(String::from(response["url"].as_str().unwrap()))
}

pub fn coub_random() -> Result<String, String> {
    let response: serde_json::Value = reqwest::Client::new()
        .get("http://coub.com/api/v2/timeline/explore/random?page=1&per_page=1")
        .send()
        .map_err(|_| "Error getting a coub right now :(")?
        .json()
        .map_err(|_| "Error getting a coub right now :(")?;

    Ok(format!("https://coub.com/embed/{}", response["coubs"][0]["permalink"].as_str().unwrap()))
}