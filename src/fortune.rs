use rand::thread_rng;
use rand::seq::IteratorRandom;
use rand::seq::SliceRandom;
use crate::web_helper::get_json;

pub fn get_fortune() -> Result<String, String> {
    let board = pick_safe_board()?;
    let content = pick_media(board)?;
    return Ok(content);
}

pub fn get_nsfw() -> Result<String, String> {
    let board = pick_not_safe_board()?;
    let content = pick_media(board)?;
    return Ok(content);
}

fn pick_media(board: String) -> Result<String, String> {
    let url = format!("http://a.4cdn.org/{}/catalog.json", board);
    let response = get_json(&url)
        .map_err(|_| "Error: Error getting a fortune media right now :(")?;

    let mut rng = thread_rng();

    let mut replies_vec = Vec::new();

    response.as_array().unwrap().iter().for_each(|page| {
        page["threads"].as_array().unwrap().iter()
        .for_each(|post| {
            if post["last_replies"].is_null() {
                return;
            }

            post["last_replies"].as_array().unwrap().iter()
               .filter(|reply| {
                   !reply["tim"].is_null() && !reply["ext"].is_null()
               })
               .for_each(|member| {
               replies_vec.push(member.clone())
            });
        })
    });
    if replies_vec.is_empty() { return Err("Error: empty replies".to_string()); }

    let reply= replies_vec.choose(&mut rng).ok_or("Error: Error getting a fortune media right now :(")?;

    return Ok(format!("http://i.4cdn.org/{}/{}{}", board, reply["tim"].as_i64().unwrap(), reply["ext"].as_str().unwrap()));
}

fn pick_safe_board() -> Result<String, String>
{
    let response = get_json("http://a.4cdn.org/boards.json")
        .map_err(|_| "Error getting a fortune board right now :(")?;

    let mut rng = thread_rng();
    let board = response["boards"].as_array().unwrap().iter()
        .filter(|board|
            board["text_only"].is_null()
            && board["ws_board"].as_i64() == Some(1)
            && board["board"].as_str() != Some("mpl")
        ).choose(&mut rng).unwrap();

    let chosen = String::from(board["board"].as_str().ok_or("Error getting a fortune board right now :(")?);

    Ok(chosen)
}

fn pick_not_safe_board() -> Result<String, String>
{
    let response: serde_json::Value = get_json("http://a.4cdn.org/boards.json")
        .map_err(|_| "Error getting a fortune board right now :(")?;

    let mut rng = thread_rng();
    let board = response["boards"].as_array().unwrap().iter()
        .filter(|board|
            board["text_only"].is_null()
            && board["ws_board"].as_i64() == Some(0)
        ).choose(&mut rng).unwrap();

    let chosen = String::from(board["board"].as_str().ok_or("Error getting a fortune board right now :(")?);

    Ok(chosen)
}
