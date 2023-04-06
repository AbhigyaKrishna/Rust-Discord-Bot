mod commands;
mod event_handler;
mod music;
mod guild_state;

use std::{
    env,
    sync::Arc
};
use dashmap::{
    DashMap,
    mapref::one::RefMut
};
use serenity::{
    Client,
    framework::standard::StandardFramework,
    prelude::{
        GatewayIntents,
        TypeMap,
        TypeMapKey
    },
};
use songbird::SerenityInit;
use tokio::sync::RwLock;
use crate::{
    commands::{
        general::GENERAL_GROUP,
        music::MUSIC_GROUP
    },
    event_handler::Handler,
    guild_state::GuildState,
    music::get_songbird_manager_throught_data
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

    let ctx = Arc::new(RwLock::new(BotContext::default()));

    let mut client = Client::builder(token, intents)
        .event_handler(Handler)
        .framework(framework)
        .register_songbird()
        .type_map_insert::<BotContextKey>(Arc::clone(&ctx))
        .await
        .expect("Error creating client");

    let mut lock = ctx.write().await;
    lock.init(&client);
    drop(lock);

    tokio::spawn(async move {
        if let Err(e) = client.start().await {
            println!("{:?}", e);
        }
    });

    let _ = tokio::signal::ctrl_c().await;
    println!("Received Ctrl-C, shutting down.");
}

pub struct BotContextKey;

impl TypeMapKey for BotContextKey {
    type Value = Arc<RwLock<BotContext>>;
}

#[derive(Default)]
pub struct BotContext {
    initialised: bool,
    data: Arc<RwLock<TypeMap>>,
    guilds: DashMap<u64, GuildState>
}

impl BotContext {
    #[inline]
    pub fn is_initialised(&self) -> bool {
        self.initialised
    }

    pub fn data(&self) -> Arc<RwLock<TypeMap>> {
        Arc::clone(&self.data)
    }

    pub fn get_guild_state<'b, G : Into<u64>>(&'b self, guild_id: G) -> Option<RefMut<'b, u64, GuildState>> {
        self.guilds.get_mut(&guild_id.into())
    }

    pub fn get_or_insert_guild_state<'b, G: Into<u64>>(&'b self, guild_id: G) -> RefMut<'b, u64, GuildState> {
        let id: u64 = guild_id.into();
        self.guilds.entry(id).or_insert(GuildState::new(id))
    }

    fn init(&mut self, client: &Client) {
        self.data = Arc::clone(&client.data);
        self.initialised = true;
    }

    async fn update(&self) {
        if !self.is_initialised() {
            return;
        }

        let songbird = get_songbird_manager_throught_data(&self.data).await;
        for mut element in self.guilds.iter_mut() {
            element.value_mut().update(&songbird).await;
        }
    }
}