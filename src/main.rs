extern crate serde_json;
extern crate memelord;

use std::{collections::{HashMap, HashSet}, env, fmt::Write, sync::Arc};
use serenity::{
    framework::standard::{
        Args, CommandOptions, CommandResult, CommandGroup,
        HelpOptions, help_commands, StandardFramework,
        macros::{command, group, help},
    },
    model::{channel::{Channel, Message}, id::UserId},
};
use serenity::prelude::{GatewayIntents, TypeMapKey};
use tokio;
use serenity::async_trait;
use serenity::client::{Client, Context, EventHandler};

mod randoms;
mod urbandict;
mod fortune;
mod web_helper;

struct Handler;

#[async_trait]
impl EventHandler for Handler {}

#[tokio::main]
async fn main() {
    start_discord().await;
}

#[group]
#[commands(dog, cat, cat2, duck, coub, whatis, whatisplain, panzer, cardinal, fortune, nsfwortune, dad)]
struct General;

struct CatApiKey;

impl TypeMapKey for CatApiKey {
    type Value = String;
}

async fn start_discord() {
    let token = env::var("DISCORD_TOKEN").unwrap();
    let prefix = env::var("DISCORD_PREFIX").unwrap();
    let cat_api_key = env::var("CATAPIKEY").unwrap();

    let intents = GatewayIntents::GUILD_MESSAGES
        | GatewayIntents::DIRECT_MESSAGES
        | GatewayIntents::MESSAGE_CONTENT;

    let framework = StandardFramework::new()
        .configure(|c| c.prefix(prefix.as_str()))
        .help(&MY_HELP)
        .group(&GENERAL_GROUP);

    let mut client = Client::builder(&token, intents)
        .event_handler(Handler)
        .framework(framework)
        .await
        .expect("Err creating client");

    {
        let mut data = client.data.write().await;
        data.insert::<CatApiKey>(String::from(cat_api_key));
    }

    if let Err(why) = client.start().await {
        println!("Client error: {:?}", why);
    }
}

#[command]
#[aliases("дог")]
async fn dog(ctx: &Context, msg: &Message) -> CommandResult {
    let res = match randoms::woof_image().await {
            Ok(img) => img,
            Err(e) => e,
        };

    msg.reply(&ctx.http, &format!("Let baba give you a doggo {}", &res)).await?;

    Ok(())
}

#[command]
#[aliases("кат2", "цат2")]
async fn cat2(ctx: &Context, msg: &Message) -> CommandResult {
    let res = match randoms::meow_image().await {
            Ok(img) => img,
            Err(e) => e,
        };

    msg.reply(&ctx.http,&format!("Let baba give you a kitteh {}", &res)).await?;
    Ok(())
}

#[command]
#[aliases("кат", "цат")]
async fn cat(ctx: &Context, msg: &Message) -> CommandResult {
    let data = ctx.data.read().await;

    let apikey = match data.get::<CatApiKey>() {
        Some(v) => v,
        None => {
            let _ = msg.reply(&ctx.http, "cat api key is invalid").await?;
            return Ok(());
        },
    };

    let res = match randoms::meow_image2(apikey).await {
        Ok(img) => img,
        Err(e) => e,
    };

    msg.reply(&ctx.http,&format!("Let baba give you a kitteh {}", &res)).await?;
    Ok(())
}

#[command]
#[aliases("патка", "дуцк", "patka")]
async fn duck(ctx: &Context, msg: &Message) -> CommandResult {
    let res = match randoms::duck_image().await {
            Ok(img) => img,
            Err(e) => e,
        };

    msg.reply(&ctx.http,&format!("Let baba give you a lucky ducky {}", &res)).await?;
    Ok(())
}

#[command]
#[aliases("цоуб")]
async fn coub(ctx: &Context, msg: &Message) -> CommandResult {
    let res = match randoms::coub_random().await {
            Ok(img) => img,
            Err(e) => e,
        };

    msg.reply(&ctx.http,&format!("Let baba give you a coub {}", &res)).await?;
    Ok(())
}

fn if_urban_is_long(result: &urbandict::UrbanResult) -> bool {
    result.description.len() > 2000 || result.example.len() > 1000
}

#[command]
#[aliases("вхатис")]
async fn whatis(ctx: &Context, msg: &Message, args: Args) -> CommandResult {
    let urban_result = match urbandict::get_term_top_embed(args.message()).await {
            Ok(def) => def,
            Err(e) => {
                let _ = msg.channel_id.say(&ctx.http, &e).await?;
                return Ok(());
            },
        };

    if if_urban_is_long(&urban_result) {
        let _ = msg.reply(&ctx.http, &urban_result.url).await?;
        return Ok(());
    }

    let _ = msg.channel_id.send_message(&ctx.http, |m| m
        .embed(|e| {
            let mut e = e
            .title(&urban_result.title)
            .description(&urban_result.description);

            if !urban_result.is_example_null() {
                e = e.field("Example", &urban_result.example, false);
            }

            e
            })
        ).await?;
    Ok(())
}

#[command]
#[aliases("вхатисплаин")]
async fn whatisplain(ctx: &Context, msg: &Message, args: Args) -> CommandResult {
    let urban_result = match urbandict::get_term_top_embed(args.message()).await {
            Ok(def) => def,
            Err(e) => {
                let _ = msg.reply(&ctx.http, &e).await?;
                return Ok(());
            },
        };

    if if_urban_is_long(&urban_result) {
        let _ = msg.reply(&ctx.http,&urban_result.url).await?;
        return Ok(());
    }

    let _ = msg.channel_id.send_message(&ctx.http, |m| {
        let example: String;
        if !urban_result.is_example_null() {
            example = format!("\nExample:\n{}", &urban_result.example);
        } else { example = "".to_string(); }

        m.content(&format!("----------\n{}\n----------\n{}{}",
            &urban_result.title,
            &urban_result.description,
            &example))
    }).await?;
    Ok(())
}

#[command]
async fn panzer(ctx: &Context, msg: &Message, args: Args) -> CommandResult {
    let img_buf = memelord::make_panzer(args.message());
    let files = vec![(&img_buf[..], "my_file.jpg")];
    let chan = msg.channel_id;
    let _ = chan.send_files(&ctx.http, files, |m| m.content(args.message())).await?;
    Ok(())
}

#[command]
async fn cardinal(ctx: &Context, msg: &Message, args: Args) -> CommandResult {
    let img_buf = memelord::make_cardinal(args.message());
    let files = vec![(&img_buf[..], "cardinal_of_rgb.jpg")];
    let _ = msg.channel_id.send_files(&ctx.http, files, |m| m.content(args.message())).await?;
    Ok(())
}

#[command]
async fn fortune(ctx: &Context, msg: &Message) -> CommandResult {
    let res = match fortune::get_fortune().await {
            Ok(img) => img,
            Err(e) => e,
        };

    let _ = msg.reply(&ctx.http,&format!("Let baba give you a 4chan {}", &res)).await?;
    Ok(())
}

#[command]
async fn nsfwortune(ctx: &Context, msg: &Message) -> CommandResult {
    if !msg.channel(&ctx).await.unwrap().is_nsfw() {
        let _ = msg.reply(&ctx.http,"Only for NSFW channels").await?;
        return Ok(());
    }

    let res = match fortune::get_nsfw().await {
            Ok(img) => img,
            Err(e) => e,
        };

    let _ = msg.reply(&ctx.http, &format!("Let baba give you a 4chan nsfw {}", &res)).await?;
    Ok(())
}

#[command]
#[aliases("дад")]
async fn dad(ctx: &Context, msg: &Message) -> CommandResult {
    let res = match randoms::dad().await {
        Ok(joke) => joke,
        Err(e) => e,
    };

    msg.reply(&ctx.http, &res).await?;

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
async fn my_help(
    context: &Context,
    msg: &Message,
    args: Args,
    help_options: &'static HelpOptions,
    groups: &[&'static CommandGroup],
    owners: HashSet<UserId>
) -> CommandResult {
    let _ = help_commands::with_embeds(context, msg, args, help_options, groups, owners).await;
    Ok(())
}