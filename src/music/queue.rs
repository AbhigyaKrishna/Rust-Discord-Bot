use std::{
    sync::Arc,
    ops::Deref
};
use tokio::sync::Mutex;
use songbird::tracks::TrackHandle;

#[derive(Debug, Clone)]
pub enum LoopState {
    None,
    Track,
    Queue,
}

impl Default for LoopState {
    fn default() -> Self {
        LoopState::None
    }
}

#[derive(Debug, Default)]
pub struct TrackQueueHandler {
    current: usize,
    queue: TrackQueue,
    loop_state: LoopState
}

impl TrackQueueHandler {
    pub async fn next(&mut self) -> Option<QueuedTrack> {
        self.current += 1;
        self.queue.get(self.current).await
    }
    
    pub async fn compute_next(&mut self) -> Option<QueuedTrack> {
        match self.loop_state {
            LoopState::None => {
                self.current += 1;
            },
            LoopState::Queue => {
                self.current = if self.current + 1 >= self.queue.size().await {
                    0
                } else {
                    self.current + 1
                };
            },
            LoopState::Track => {}
        }
        self.queue.get(self.current).await
    }
    
    pub async fn previous(&mut self) -> Option<QueuedTrack> {
        self.current = if self.current - 1 <= 0 {
            0
        } else {
            self.current - 1
        };
        self.queue.get(self.current).await
    }
    
    pub async fn enqueue(&mut self, track: TrackHandle) {
        self.queue.0.lock().await.push(QueuedTrack(track))
    }
}

#[derive(Debug, Clone, Default)]
pub struct TrackQueue(Arc<Mutex<Vec<QueuedTrack>>>);

impl TrackQueue {
    pub async fn size(&self) -> usize {
        self.0.lock().await.len()
    }
    
    pub async fn get(&self, index: usize) -> Option<QueuedTrack> {
        self.0.lock().await.get(index).cloned()
    }
}

#[derive(Debug, Clone)]
pub struct QueuedTrack(TrackHandle);

impl Deref for QueuedTrack {
    type Target = TrackHandle;
    
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl QueuedTrack {
    fn handle(&self) -> TrackHandle {
        self.0.clone()
    }
}