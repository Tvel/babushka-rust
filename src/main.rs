#[macro_use] extern crate serenity;
extern crate serde_json;

use serenity::client::Client;
use serenity::prelude::EventHandler;
use serenity::framework::standard::StandardFramework;
use serde_json::{Value};
use std::fs::File;
use std::io::Read;
//use std::env;

mod randoms;
mod urbandict;

struct Handler;

impl EventHandler for Handler {}

fn main() {
    let v = load_settings();
    start_discord(&v);
}

fn load_settings() -> Value
{
    let mut file = File::open("settings.json").unwrap();
    let mut data = String::new();
    file.read_to_string(&mut data).unwrap();
    serde_json::from_str(&data).unwrap()
}

fn start_discord(settings: &Value) {
    let token = settings["token"].as_str().unwrap();
    let prefix = settings["prefix"].as_str().unwrap();
    
    // Login with a bot token from the environment
    let mut client = Client::new(token, Handler)
        .expect("Error creating client");
    client.with_framework(StandardFramework::new()
        .configure(|c| c.prefix(prefix)) 
        .cmd("dog", dog)
        .cmd("дог", dog)
        .cmd("cat", cat)
        .cmd("цат", cat)
        .cmd("coub", coub)
        .cmd("цоуб", coub)
        .cmd("whatis", ub)
        .cmd("вхатис", ub)
        .cmd("whatisplain", ub_plain)
        .cmd("вхатисплаин", ub_plain)
        );

    // start listening for events by starting a single shard
    if let Err(why) = client.start() {
        println!("An error occurred while running the client: {:?}", why);
    }
}

command!(dog(_context, message) {
    let mut res = match randoms::woof_image() {
            Ok(img) => img,
            Err(e) => e,
        };

    let _ = message.reply(&format!("Let baba give you a doggo {}", &res));
});

command!(cat(_context, message) {
    let mut res = match randoms::meow_image() {
            Ok(img) => img,
            Err(e) => e,
        };

    let _ = message.reply(&format!("Let baba give you a kitteh {}", &res));
});

command!(coub(_context, message) {
    let mut res = match randoms::coub_random() {
            Ok(img) => img,
            Err(e) => e,
        };

    let _ = message.reply(&format!("Let baba give you a coub {}", &res));
});

command!(ub(_context, message, args) {
    let mut urban_result = match urbandict::get_term_top_embed(args.full()) {
            Ok(def) => def,
            Err(e) => {
                let _ = message.reply(&e);
                return Ok(());
            },
        };

    if urban_result.description.len() > 2000 || urban_result.example.len() > 1000 {
        let _ = message.reply(&urban_result.url);
        return Ok(());
    }

    let _ = message.channel_id.send_message(|m| m
        .embed(|e| {
            let mut e = e
            .title(&urban_result.title)
            .description(&urban_result.description);

            if !urban_result.is_example_null() {
                e = e.field("Example", &urban_result.example, false);
            }

            e
            })
        );
});

command!(ub_plain(_context, message, args) {
    let mut urban_result = match urbandict::get_term_top_embed(args.full()) {
            Ok(def) => def,
            Err(e) => {
                let _ = message.reply(&e);
                return Ok(());
            },
        };

    if urban_result.description.len() > 2000 || urban_result.example.len() > 1000 {
        let _ = message.reply(&urban_result.url);
        return Ok(());
    }

    let _ = message.channel_id.send_message(|m| {
        let mut example: String;
        if !urban_result.is_example_null() {
            example = format!("\nExample:\n{}", &urban_result.example);
        } else { example = "".to_string(); }

        m.content(&format!("----------\n{}\n----------\n{}{}",
            &urban_result.title,
            &urban_result.description,
            &example))
    });
});