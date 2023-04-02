use std::env;
use serenity::Client;
use serenity::framework::standard::StandardFramework;
use serenity::prelude::GatewayIntents;
use songbird::SerenityInit;
use crate::commands::general::GENERAL_GROUP;
use crate::commands::music::MUSIC_GROUP;
use crate::event_handler::Handler;

mod commands;
mod event_handler;
mod music;

#[tokio::main]
async fn main() {
    let framework = StandardFramework::new()
        .configure(|c| c.prefix("-"))
        .group(&GENERAL_GROUP)
        .group(&MUSIC_GROUP);

    // Login with a bot token from the environment
    let token = env::args().nth(1).expect("Expected a bot token");
    let intents = GatewayIntents::non_privileged() |
        GatewayIntents::MESSAGE_CONTENT | GatewayIntents::GUILD_VOICE_STATES;
    let mut client = Client::builder(token, intents)
        .event_handler(Handler)
        .framework(framework)
        .register_songbird()
        .await
        .expect("Error creating client");

    tokio::spawn(async move {
        let _ = client.start().await.map_err(|why| println!("Client ended: {:?}", why));
    });

    tokio::signal::ctrl_c().await;
    println!("Received Ctrl-C, shutting down.");
}
