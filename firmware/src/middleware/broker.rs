use super::message::Subscriber;
use super::message::Callback;
use super::message::Topics;
use super::message::Message;
use super::publisher::PublisherManager;
use super::subscriber::SubscriberManager;

use std::sync::Mutex;

pub struct Broker {
    subscribers: Mutex<Vec<Subscriber>>
}

impl PublisherManager for Broker {
    fn publish(&self, message: &mut Message<'_>) {
        Broker::publish(self, message);
    }
}

impl SubscriberManager for Broker {
    fn subscribe(&self, callback: Callback, topic: Topics, delete: bool) {
        Broker::subscribe(self, callback, topic, delete);
    }
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

    pub fn publish(&self, message: &mut Message) {

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