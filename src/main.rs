#[macro_use] extern crate serenity;
extern crate serde_json;

use serenity::client::Client;
use serenity::prelude::EventHandler;
use serenity::framework::standard::StandardFramework;
use serde_json::{Value};
use std::fs::File;
use std::io::Read;
//use std::env;

mod animals;

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
    return serde_json::from_str(&data).unwrap();
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
        .cmd("cat", cat)
        );

    // start listening for events by starting a single shard
    if let Err(why) = client.start() {
        println!("An error occurred while running the client: {:?}", why);
    }
}

command!(dog(_context, message) {
    let mut res = match animals::woof_image() {
            Ok(img) => img,
            Err(e) => e,
        };

    let _ = message.reply(&res);
});

command!(cat(_context, message) {
    let mut res = match animals::meow_image() {
            Ok(img) => img,
            Err(e) => e,
        };

    let _ = message.reply(&res);
});