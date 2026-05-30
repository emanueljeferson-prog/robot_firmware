use super::message::Subscriber;
use super::message::Callback;
use super::message::Topics;
use super::message::Message;

pub struct Broker {
    subscribers: Vec<Subscriber>
}

impl Broker {
    pub fn new() -> Self {
        Self { subscribers: Vec::new() }
    }

    pub fn subscribe(&mut self, callback: Callback, topic: Topics) {
        self.subscribers.push(
            Subscriber::new(callback, topic)
        );
    }

    pub fn publish(&self, message: &Message) {
        self.subscribers
            .iter()                                                         // iterator
            .filter( |subscriber| { subscriber.topic == message.topic() } ) // filter by equal topic
            .for_each( |subscriber| { (subscriber.callback)(message) })     // send the message to the subscriber
    }
}