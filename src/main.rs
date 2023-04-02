use serenity::async_trait;
use serenity::model::prelude::Activity;
use serenity::prelude::*;
use serenity::model::channel::Message;
use serenity::framework::standard::macros::{command, group};
use serenity::framework::standard::{StandardFramework, CommandResult};

#[group]
#[commands(ping, seggs)]
struct General;

struct Handler;

#[async_trait]
impl EventHandler for Handler {}

#[tokio::main]
async fn main() {
    let framework = StandardFramework::new()
        .configure(|c| c.prefix("-"))
        .group(&GENERAL_GROUP);

    // Login with a bot token from the environment
    let token = "ODA2MTY1MzY1NzM4NTY5NzU5.GQFG4K.DGOJIsEL7CNY1vQ0Pvzl2UYDo5y5Yk96yd04TM";
    let intents = GatewayIntents::non_privileged() | GatewayIntents::MESSAGE_CONTENT;
    let mut client = Client::builder(token, intents)
        .event_handler(Handler)
        .framework(framework)
        .await
        .expect("Error creating client");

    // start listening for events by starting a single shard
    if let Err(why) = client.start().await {
        println!("An error occurred while running the client: {:?}", why);
    }
}

#[command]
async fn ping(ctx: &Context, msg: &Message) -> CommandResult {
    msg.reply(ctx, "Pong!").await?;

    Ok(())
}

#[command]
async fn seggs(ctx: &Context, msg: &Message) -> CommandResult {
    msg.reply(ctx, format!("<@{}> seggs", msg.author.id.as_u64())).await?;

    Ok(())
}

#[command]
async fn activity(ctx: &Context, msg: &Message) -> CommandResult {
    let content: Vec<&str> = msg.content.clone().split_whitespace().collect();
    if content.len() < 3 {
        msg.reply(ctx, "Invalid input!");
        return Ok(());
    }

    match content[1] {
        "playing" =>
    }
    Ok(())
}
