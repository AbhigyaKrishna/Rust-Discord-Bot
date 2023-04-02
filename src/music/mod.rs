mod track;

use std::collections::VecDeque;
use std::sync::Arc;
use std::sync::atomic::AtomicUsize;
use serenity::http::client::Http;
use serenity::model::prelude::ChannelId;
use serenity::prelude::Context;
use songbird::id::{ChannelId as SChannelId, GuildId as SGuildId};
use songbird::{input, Songbird};
use songbird::input::Input;

#[derive(Debug)]
pub(crate) struct TrackQueue {
    queue: VecDeque<Input>
}

impl TrackQueue {
    pub fn new() -> Self {
        Self {
            queue: VecDeque::new()
        }
    }
}

impl Default for TrackQueue {
    fn default() -> Self {
        Self::new()
    }
}

pub(crate) struct TrackEndNotifier {
    pub channel_id: ChannelId,
    pub http: Arc<Http>
}

pub(crate) struct ChannelDurationNotifier {
    pub channel_id: ChannelId,
    pub count: Arc<AtomicUsize>,
    pub http: Arc<Http>
}

pub(crate) struct SongEndNotifier {
    pub channel_id: ChannelId,
    pub http: Arc<Http>
}

pub async fn get_songbird_manager(ctx: &Context) -> Arc<Songbird> {
    songbird::get(ctx).await
        .expect("Songbird Voice client placed in at initialisation.").clone()
}