use super::message::Subscriber;
use super::message::Callback;
use super::message::Topics;
use super::message::Message;
use std::sync::Mutex;

pub struct Broker {
    subscribers: Mutex<Vec<Subscriber>>
}

impl Broker {
    pub fn new() -> Self {
        Self { subscribers: Mutex::new(Vec::new()) }
    }

    pub fn run(&self) {
        self.delete_sub();
    }

    pub fn subscribe(&self, callback: Callback, topic: Topics, delete: bool) {
        self.subscribers
            .lock()
            .unwrap()
            .push( Subscriber::new(callback, topic, delete) );
    }

    pub fn publish(&self, message: &Message) {

        let subscribers =
            self.subscribers
                .lock()
                .unwrap();

        let callbacks = subscribers
            .iter()
            .filter(|subscriber| {
                subscriber.topic ==
                message.topic()
            })
            .map(|subscriber| {
                subscriber.callback.clone()
            })
            .collect::<Vec<_>>();

        drop(subscribers); // unlock explícito

        for callback in callbacks {
            callback(message);
        }
    }

    pub fn delete_sub(&self) {
        self.subscribers.lock().unwrap().retain(|sub| { !sub.delete_after });
    }
}