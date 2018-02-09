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
    let response = match requests::get("https://random.cat/meow") {
         Ok(res) => res,
         Err(_) => return Err(String::from("Error getting a kitteh right now :("))
    };

    if response.status_code() != requests::StatusCode::Ok {
        return Err(String::from("Cannot get a kitteh right now :("));
    }
    let data = response.json().unwrap();

    Ok(data["file"].to_string())
}