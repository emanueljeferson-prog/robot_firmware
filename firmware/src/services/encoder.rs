use crate::middleware::message::Message;
use crate::middleware::message::Topics;
use crate::middleware::subscriber::SubscriberManager;

use alloc::sync::Arc;

pub struct EncoderService {

}

impl EncoderService {
    pub fn new() -> Self {
        Self { }
    }

    pub fn init(&self, middleware: &dyn SubscriberManager) {
        middleware.subscribe(
            Arc::new(|msg: &mut Message| {
                if let Message::ReadSpeed(speed) = msg {
                    **speed = EncoderService::read_speed();
                }
            }),
            Topics::ReadSpeed,
            false
        )
    }

    pub fn read_speed() -> f32 {
        let speed_read: f32 = 55.11; 
        speed_read
    }
}