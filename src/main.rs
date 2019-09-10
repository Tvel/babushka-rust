extern crate serde_json;
extern crate memelord;

use std::{collections::{HashMap, HashSet}, env, fmt::Write, sync::Arc};
use serenity::{
    client::bridge::gateway::{ShardId, ShardManager},
    framework::standard::{
        Args, CheckResult, CommandOptions, CommandResult, CommandGroup,
        DispatchError, HelpOptions, help_commands, StandardFramework,
        macros::{command, group, help, check},
    },
    model::{channel::{Channel, Message}, gateway::Ready, id::UserId},
    utils::{content_safe, ContentSafeOptions},
};

// This imports `typemap`'s `Key` as `TypeMapKey`.
use serenity::prelude::*;

use serenity::client::Client;
use serenity::prelude::EventHandler;
use serde_json::{Value};
use std::fs::File;
use std::io::Read;
//use std::env;

mod randoms;
mod urbandict;
mod fortune;
mod web_helper;

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

group!({
    name: "general",
    options: {},
    commands: [dog, cat, duck, coub, whatis, whatisplain, panzer, cardinal, fortune, nsfwortune]
});

fn start_discord(settings: &Value) {
    let token = settings["token"].as_str().unwrap();
    let prefix = settings["prefix"].as_str().unwrap();
    
    // Login with a bot token from the environment
    let mut client = Client::new(token, Handler)
        .expect("Error creating client");
    client.with_framework(StandardFramework::new()
        .configure(|c| c.prefix(prefix))
        .help(&MY_HELP)
        .group(&GENERAL_GROUP)
    );

    // start listening for events by starting a single shard
    if let Err(why) = client.start() {
        println!("An error occurred while running the client: {:?}", why);
    }
}

#[command]
#[aliases("дог")]
fn dog(ctx: &mut Context, msg: &Message) -> CommandResult {
    let res = match randoms::woof_image() {
            Ok(img) => img,
            Err(e) => e,
        };

    let _ = msg.reply(&ctx,&format!("Let baba give you a doggo {}", &res));
    Ok(())
}

#[command]
#[aliases("кат", "цат")]
fn cat(ctx: &mut Context, msg: &Message) -> CommandResult {
    let res = match randoms::meow_image() {
            Ok(img) => img,
            Err(e) => e,
        };

    let _ = msg.reply(&ctx,&format!("Let baba give you a kitteh {}", &res));
    Ok(())
}

#[command]
#[aliases("патка")]
fn duck(ctx: &mut Context, msg: &Message) -> CommandResult {
    let res = match randoms::duck_image() {
            Ok(img) => img,
            Err(e) => e,
        };

    let _ = msg.reply(&ctx,&format!("Let baba give you a lucky ducky {}", &res));
    Ok(())
}

#[command]
#[aliases("цоуб")]
fn coub(ctx: &mut Context, msg: &Message) -> CommandResult {
    let res = match randoms::coub_random() {
            Ok(img) => img,
            Err(e) => e,
        };

    let _ = msg.reply(&ctx,&format!("Let baba give you a coub {}", &res));
    Ok(())
}

#[command]
#[aliases("вхатис")]
fn whatis(ctx: &mut Context, msg: &Message, args: Args) -> CommandResult {
    let urban_result = match urbandict::get_term_top_embed(args.message()) {
            Ok(def) => def,
            Err(e) => {
                let _ = msg.reply(&ctx, &e);
                return Ok(());
            },
        };

    if urban_result.description.len() > 2000 || urban_result.example.len() > 1000 {
        let _ = msg.reply(&ctx, &urban_result.url);
        return Ok(());
    }

    let _ = msg.channel_id.send_message(&ctx, |m| m
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
    Ok(())
}

#[command]
#[aliases("вхатисплаин")]
fn whatisplain(ctx: &mut Context, msg: &Message, args: Args) -> CommandResult {
    let urban_result = match urbandict::get_term_top_embed(args.message()) {
            Ok(def) => def,
            Err(e) => {
                let _ = msg.reply(&ctx, &e);
                return Ok(());
            },
        };

    if urban_result.description.len() > 2000 || urban_result.example.len() > 1000 {
        let _ = msg.reply(&ctx,&urban_result.url);
        return Ok(());
    }

    let _ = msg.channel_id.send_message(&ctx, |m| {
        let example: String;
        if !urban_result.is_example_null() {
            example = format!("\nExample:\n{}", &urban_result.example);
        } else { example = "".to_string(); }

        m.content(&format!("----------\n{}\n----------\n{}{}",
            &urban_result.title,
            &urban_result.description,
            &example))
    });
    Ok(())
}

#[command]
fn panzer(ctx: &mut Context, msg: &Message, args: Args) -> CommandResult {
    let img_buf = memelord::make_panzer(args.message());
    let files = vec![(&img_buf[..], "my_file.jpg")];
    let chan = msg.channel_id;
    let _ = chan.send_files(&ctx, files, |m| m.content(args.message()));
    Ok(())
}

#[command]
fn cardinal(ctx: &mut Context, msg: &Message, args: Args) -> CommandResult {
    let img_buf = memelord::make_cardinal(args.message());
    let files = vec![(&img_buf[..], "cardinal_of_rgb.jpg")];
    let _ = msg.channel_id.send_files(&ctx, files, |m| m.content(args.message()));
    Ok(())
}

#[command]
fn fortune(ctx: &mut Context, msg: &Message) -> CommandResult {
    let res = match fortune::get_fortune() {
            Ok(img) => img,
            Err(e) => e,
        };

    let _ = msg.reply(&ctx,&format!("Let baba give you a 4chan {}", &res));
    Ok(())
}

#[command]
fn nsfwortune(ctx: &mut Context, msg: &Message) -> CommandResult {
    if !msg.channel(&ctx).unwrap().is_nsfw() {
        let _ = msg.reply(&ctx,"Only for NSFW channels");
        return Ok(());
    }

    let res = match fortune::get_nsfw() {
            Ok(img) => img,
            Err(e) => e,
        };

    let _ = msg.reply(&ctx, &format!("Let baba give you a 4chan nsfw {}", &res));
    Ok(())
}


#[help]
#[individual_command_tip =
"Hi\n\
If you want more information about a specific command, just pass the command as argument."]
#[command_not_found_text = "Could not find: `{}`."]
#[max_levenshtein_distance(3)]
#[indention_prefix = "+"]
#[lacking_permissions = "Hide"]
#[lacking_role = "Nothing"]
#[wrong_channel = "Strike"]
fn my_help(
    context: &mut Context,
    msg: &Message,
    args: Args,
    help_options: &'static HelpOptions,
    groups: &[&'static CommandGroup],
    owners: HashSet<UserId>
) -> CommandResult {
    help_commands::with_embeds(context, msg, args, help_options, groups, owners)
}