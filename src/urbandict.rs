extern crate requests;
use self::requests::ToJson;

pub struct UrbanResult {
    pub title: String,
    pub description: String,
    pub example: String,
    pub url: String
}

impl UrbanResult {
    fn new(title: String, description: String, example: String, url: String) -> UrbanResult {
        UrbanResult { title, description, example, url }
    }

    pub fn is_example_null(&self) -> bool {
        self.example.is_empty() || self.example.eq("null")
    }
}

#[allow(dead_code)]
pub fn get_term_top_definition(term: &str) -> Result<String, String> {
    let response = match requests::get(format!("http://api.urbandictionary.com/v0/define?term={}", term)) {
        Ok(res) => res,
        Err(_) => return Err(String::from("Error getting a definition right now :("))
    };

    if response.status_code() != requests::StatusCode::Ok {
        return Err(String::from("Cannot get a definition right now :("));
    }
    let data = response.json().unwrap();

    Ok(data["list"][0]["definition"].to_string())
}

pub fn get_term_top_embed(term: &str) -> Result<UrbanResult, String> {
    let response = match requests::get(format!("http://api.urbandictionary.com/v0/define?term={}", term)) {
        Ok(res) => res,
        Err(_) => return Err(String::from("Error getting a definition right now :("))
    };

    if response.status_code() != requests::StatusCode::Ok {
        return Err(String::from("Cannot get a definition right now :("));
    }

    let data = response.json().unwrap();
    let def = &data["list"][0];

    let word = def["word"].to_string();
    if word.eq("null") {
        return Err(String::from("No idea"));
    }

    let ur = UrbanResult::new(
        word,
        def["definition"].to_string(),
        def["example"].to_string(),
        def["permalink"].to_string());

    return Ok(ur);
}