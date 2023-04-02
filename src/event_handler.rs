use std::sync::atomic::Ordering;
use serenity::async_trait;
use serenity::model::prelude::Ready;
use serenity::prelude::{Context, EventHandler};
use songbird::{Event, EventContext, EventHandler as VoiceEventHandler};
use crate::music::{ChannelDurationNotifier, SongEndNotifier, TrackEndNotifier};

pub(crate) struct Handler;

#[async_trait]
impl EventHandler for Handler {

    async fn ready(&self, _ctx: Context, data: Ready) {
        println!("{}#{} is connected!", data.user.name, data.user.discriminator);
    }

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

#[async_trait]
impl VoiceEventHandler for ChannelDurationNotifier {

    async fn act(&self, _ctx: &EventContext<'_>) -> Option<Event> {
        let count_before = self.count.fetch_add(1, Ordering::Relaxed);
        let _ = self.channel_id.say(&self.http, format!("I've been in the channel for {} minutes!", count_before + 1)).await;

        None
    }

}

#[async_trait]
impl VoiceEventHandler for SongEndNotifier {

    async fn act(&self, _ctx: &EventContext<'_>) -> Option<Event> {
        let _ = self.channel_id.say(&self.http, "Song ended.").await;

        None
    }

}