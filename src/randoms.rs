//extern crate serde_json;

use crate::web_helper::get_json;
use crate::web_helper::get_plain;

pub fn woof_image() -> Result<String, String> {
    let response = get_plain("https://random.dog/woof")
        .map_err(|_| "Error getting a doggo right now :(")?;

    let url = format!("https://random.dog/{}", response);
    Ok(url)
}

pub fn meow_image() -> Result<String, String> {
    let response = get_json("http://aws.random.cat/meow")
        .map_err(|_| "Error getting a kitteh right now :(")?;

    Ok(String::from(response["file"].as_str().unwrap()))
}

pub fn duck_image() -> Result<String, String> {
    let response = get_json("https://random-d.uk/api/v1/random")
        .map_err(|_| "Error getting a ducky right now :(")?;

    Ok(String::from(response["url"].as_str().unwrap()))
}

pub fn coub_random() -> Result<String, String> {
    let response = get_json("http://coub.com/api/v2/timeline/explore/random?page=1&per_page=1")
        .map_err(|_| "Error getting a coub right now :(")?;

    Ok(format!("https://coub.com/embed/{}", response["coubs"][0]["permalink"].as_str().unwrap()))
}