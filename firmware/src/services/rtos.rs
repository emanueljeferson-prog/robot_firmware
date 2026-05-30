use crate::middleware::broker::Broker;
use crate::middleware::message::Message;
use crate::middleware::message::Topics;
use crate::middleware::message::TaskDescriptor;
use std::sync::Arc;

pub struct RtosService;

impl RtosService {
    pub fn init(middleware: &Broker) {
        middleware.subscribe(
            Arc::new(|msg: &Message| {
                if let Message::RegisterTask(desc) = msg {
                    RtosService::crete_task(desc);
                }
            }),
            Topics::RegisterTask
        );
    }
    
    pub fn crete_task(desc: &TaskDescriptor) {
        println!("Descriptor: {:#?}", desc);
    }
}