use crate::middleware::message::Topics;
use crate::middleware::message::Callback;

pub trait SubscriberManager {
    fn subscribe(&self, callback: Callback, topic: Topics, delete: bool);
}