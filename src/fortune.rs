extern crate requests;
use self::requests::ToJson;
extern crate rand;
use rand::thread_rng;
use rand::seq::IteratorRandom;
use rand::seq::SliceRandom;

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
    let response = match requests::get(format!("http://a.4cdn.org/{}/catalog.json", board)) {
        Ok(res) => res,
        Err(_) => return Err(String::from("Error: Error getting a fortune media right now :("))
    };
    if response.status_code() != requests::StatusCode::Ok {
        return Err(String::from("Error: Error getting a fortune board right now :("));
    }
    let data = response.json().unwrap();
    let mut rng = thread_rng();

    let mut replies_vec = Vec::new();
    data.members().for_each(|page| {
        page["threads"].members()
        .for_each(|post| {
           post["last_replies"].members()
               .filter(|reply| {
                   !reply["tim"].is_null() && !reply["ext"].is_null()
               })
               .for_each(|member| {
               replies_vec.push(member.clone())
            });
        })
    });
    if replies_vec.is_empty() { return Err("Error: empty replies".to_string()); }

    let reply= replies_vec.choose(&mut rng).unwrap();

    return Ok(format!("http://i.4cdn.org/{}/{}{}", board, reply["tim"].to_string(), reply["ext"]));
}

fn pick_safe_board() -> Result<String, String>
{
    let response = match requests::get("http://a.4cdn.org/boards.json") {
        Ok(res) => res,
        Err(_) => return Err(String::from("Error getting a fortune board right now :("))
    };

    if response.status_code() != requests::StatusCode::Ok {
        return Err(String::from("Error getting a fortune board right now :("));
    }
    let data = response.json().unwrap();
    let mut rng = thread_rng();
    let board = data["boards"].members()
        .filter(|board|
            board["text_only"].is_null()
            && board["ws_board"].as_i32() == Some(1)
            && board["board"].as_str() != Some("mpl")
        ).choose(&mut rng).unwrap();

    Ok(board["board"].to_string())
}

fn pick_not_safe_board() -> Result<String, String>
{
    let response = match requests::get("http://a.4cdn.org/boards.json") {
        Ok(res) => res,
        Err(_) => return Err(String::from("Error getting a fortune board right now :("))
    };

    if response.status_code() != requests::StatusCode::Ok {
        return Err(String::from("Error getting a fortune board right now :("));
    }
    let data = response.json().unwrap();
    let mut rng = thread_rng();
    let board = data["boards"].members()
        .filter(|board|
            board["text_only"].is_null()
            && board["ws_board"].as_i32() == Some(0)
        ).choose(&mut rng).unwrap();

    Ok(board["board"].to_string())
}