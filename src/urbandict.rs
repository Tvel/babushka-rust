extern crate url;
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

    let response: serde_json::Value = reqwest::Client::new()
        .get(&url)
        .send()
        .map_err(|_| "Error getting a definition right now :(")?
        .json()
        .map_err(|_| "Error getting a definition right now :(")?;

    Ok(response["list"][0]["definition"].to_string())
}

pub fn get_term_top_embed(term: &str) -> Result<UrbanResult, String> {
    let url : String = form_urlencoded::Serializer::new(String::from("http://api.urbandictionary.com/v0/define?"))
        .append_pair("term", term).finish();
    let response: serde_json::Value = reqwest::Client::new()
        .get(&url)
        .send()
        .map_err(|_| "Error getting a definition right now :(")?
        .json()
        .map_err(|_| "Error getting a definition right now :(")?;
    let def = &response["list"][0];

    let word = String::from(def["word"].as_str().ok_or(String::from("No idea"))?);

    let ur = UrbanResult::new(
        word,
        String::from(def["definition"].as_str().unwrap()),
        String::from(def["example"].as_str().unwrap()),
        String::from(def["permalink"].as_str().unwrap()));

    return Ok(ur);
}