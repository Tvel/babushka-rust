extern crate requests;
extern crate url;
use self::requests::ToJson;
use self::url::form_urlencoded;

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
    let url : String = form_urlencoded::Serializer::new(String::from("http://api.urbandictionary.com/v0/define?"))
        .append_pair("term", term).finish();
    let response = match requests::get(url) {
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
    let url : String = form_urlencoded::Serializer::new(String::from("http://api.urbandictionary.com/v0/define?"))
        .append_pair("term", term).finish();
    let response = match requests::get(url) {
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