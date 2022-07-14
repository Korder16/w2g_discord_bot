mod w2g_api;
use w2g_api::create_room;

use std::env;

use serenity::async_trait;
use serenity::prelude::*;
use serenity::model::channel::Message;
use serenity::framework::standard::macros::{command, group};
use serenity::framework::standard::{StandardFramework, CommandResult};


#[group]
#[commands(watch)]
struct General;

struct Handler;

#[async_trait]
impl EventHandler for Handler {}

#[tokio::main]
async fn main() {
    let framework = StandardFramework::new()
        .configure(|c| c.prefix("!"))
        .group(&GENERAL_GROUP);

    let token = env::var("DISCORD_W2G_BOT_TOKEN").expect("Expected a token in the environment");
    let intents = GatewayIntents::non_privileged() | GatewayIntents::MESSAGE_CONTENT;

    let mut client = Client::builder(token, intents)
        .event_handler(Handler)
        .framework(framework)
        .await
        .expect("Error creating client");

    if let Err(why) = client.start().await {
        println!("Client error: {:?}", why);
    }
}

#[command]
async fn watch(ctx: &Context, msg: &Message) -> CommandResult {
    let tokens: Vec<&str> = msg.content.split_whitespace().collect();
    let video_url = tokens.last().copied().unwrap();
    let room_url = create_room(video_url).await?;
    println!("Created room: https://w2g.tv/rooms/{}", room_url);
    msg.reply(ctx, format!("https://w2g.tv/rooms/{}", room_url)).await?;

    Ok(())
}