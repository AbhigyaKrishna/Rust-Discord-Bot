pub mod queue;

use std::sync::Arc;
use tokio::sync::RwLock;
use serenity::{
    async_trait,
    http::client::Http,
    model::prelude::ChannelId,
    prelude::{
        Context,
        TypeMap
    }
};
use songbird::{
    Event,
    EventContext,
    EventHandler as VoiceEventHandler,
    Songbird,
    SongbirdKey
};

pub(crate) struct TrackEndNotifier {
    pub channel_id: ChannelId,
    pub http: Arc<Http>
}

#[async_trait]
impl VoiceEventHandler for TrackEndNotifier {
    async fn act(&self, ctx: &EventContext<'_>) -> Option<Event> {
        if let EventContext::Track(track_list) = ctx {
            let _ = self.channel_id.say(&self.http, format!("Track ended: {}.", track_list.len())).await;
        }

        None
    }
}

pub(crate) struct SongEndNotifier {
    pub channel_id: ChannelId,
    pub http: Arc<Http>
}

#[async_trait]
impl VoiceEventHandler for SongEndNotifier {
    async fn act(&self, _ctx: &EventContext<'_>) -> Option<Event> {
        let _ = self.channel_id.say(&self.http, "Song ended.").await;

        None
    }
}

pub async fn get_songbird_manager_throught_data(data: &Arc<RwLock<TypeMap>>) -> Arc<Songbird> {
    let data = data.read().await;
    data.get::<SongbirdKey>().cloned()
    .expect("Songbird Voice client placed in at initialisation.").clone()
}

pub async fn get_songbird_manager(ctx: &Context) -> Arc<Songbird> {
    songbird::get(ctx).await
        .expect("Songbird Voice client placed in at initialisation.").clone()
}