mod w2g_api_client;
use w2g_api_client::create_room;
use std::env;
use dotenv::dotenv;

use serenity::async_trait;
use serenity::framework::standard::macros::{command, group};
use serenity::framework::standard::{CommandResult, StandardFramework};
use serenity::model::channel::Message;
use serenity::prelude::*;

use log::warn;
use simple_logger::SimpleLogger;

#[group]
#[commands(watch)]
struct General;

struct Handler;

#[async_trait]
impl EventHandler for Handler {}

#[tokio::main]
async fn main() {
    SimpleLogger::new().with_level(log::LevelFilter::Warn).init().unwrap();
    dotenv().ok();

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

fn get_video_url(msg: &Message) -> String {
    let tokens: Vec<&str> = msg.content.split_whitespace().collect();
    String::from(tokens.last().copied().unwrap())
}

#[command]
async fn watch(ctx: &Context, msg: &Message) -> CommandResult {
    let video_url: String = get_video_url(msg);

    let room = create_room(video_url.as_str()).await?;

    let room_url = format!("https://w2g.tv/rooms/{}", room.streamkey);
    warn!("Created room: {}", &room_url);
    msg.reply(ctx, room_url).await?;

    Ok(())
}
