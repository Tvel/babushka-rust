extern crate requests;
use self::requests::ToJson;

pub fn woof_image() -> Result<String, String> {
    let response = match requests::get("https://random.dog/woof") {
         Ok(res) => res,
         Err(_) => return Err(String::from("Error getting a doggo right now :("))
    };

    if response.status_code() != requests::StatusCode::Ok {
        return Err(String::from("Cannot get a doggo right now :("));
    }

    let data = response.text().unwrap();

    let url = format!("https://random.dog/{}", data);
    Ok(url)
}

pub fn meow_image() -> Result<String, String> {
    let response = match requests::get("http://aws.random.cat/meow") {
         Ok(res) => res,
         Err(_) => return Err(String::from("Error getting a kitteh right now :("))
    };

    if response.status_code() != requests::StatusCode::Ok {
        return Err(String::from("Cannot get a kitteh right now :("));
    }
    let data = response.json().unwrap();

    Ok(data["file"].to_string())
}

pub fn duck_image() -> Result<String, String> {
    let response = match requests::get("https://random-d.uk/api/v1/random") {
        Ok(res) => res,
        Err(_) => return Err(String::from("Error getting a ducky right now :("))
    };

    if response.status_code() != requests::StatusCode::Ok {
        return Err(String::from("Cannot get a ducky right now :("));
    }
    let data = response.json().unwrap();

    Ok(data["url"].to_string())
}

pub fn coub_random() -> Result<String, String> {
    let response = match requests::get("http://coub.com/api/v2/timeline/explore/random?page=1&per_page=1") {
        Ok(res) => res,
        Err(_) => return Err(String::from("Error getting a coub right now :("))
    };

    if response.status_code() != requests::StatusCode::Ok {
        return Err(String::from("Cannot get a coub right now :("));
    }
    let data = response.json().unwrap();

    Ok(format!("https://coub.com/embed/{}", data["coubs"][0]["permalink"].to_string()))
}