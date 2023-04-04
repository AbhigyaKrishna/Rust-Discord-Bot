mod commands;
mod event_handler;
mod music;

use std::env;
use serenity::{
    Client,
    framework::standard::StandardFramework,
    prelude::GatewayIntents,
};
use songbird::SerenityInit;
use crate::{
    commands::{
        general::GENERAL_GROUP,
        music::MUSIC_GROUP
    },
    event_handler::Handler,
};

#[tokio::main]
async fn main() {
    let framework = StandardFramework::new()
        .configure(|c| c.prefix("-"))
        .group(&GENERAL_GROUP)
        .group(&MUSIC_GROUP);

    // Login with a bot token from the environment
    let token = env::args().last().expect("Expected a bot token");
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
