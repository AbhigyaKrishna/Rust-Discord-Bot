use std::sync::Arc;
use serenity::prelude::Context;
use songbird::Songbird;
use crate::music::{
    queue::TrackQueueHandler,
    get_songbird_manager
};

#[derive(Debug)]
pub struct GuildState {
    id: u64,
    queue_handler: TrackQueueHandler,
    self_mute: bool,
    self_deafen: bool
}

impl GuildState {
    pub fn new(id: u64) -> Self {
        Self {
            id,
            queue_handler: Default::default(),
            self_mute: false,
            self_deafen: false
        }
    }

    pub fn id(&self) -> u64 {
        self.id
    }

    pub fn queue(&self) -> &TrackQueueHandler {
        &self.queue_handler
    }

    pub fn is_mute(&self) -> bool {
        self.self_mute
    }

    pub fn is_deafen(&self) -> bool {
        self.self_deafen
    }

    pub(crate) async fn update_state(&mut self, ctx: &Context) {
        let manager = get_songbird_manager(ctx).await;
        self.update(&manager).await
    }

    pub(crate) async fn update(&mut self, songbird: &Arc<Songbird>) {
        if let Some(handler) = songbird.get(self.id) {
            let call = handler.lock().await;
            self.self_mute = call.is_mute();
            self.self_deafen = call.is_deaf();
        }
    }
}