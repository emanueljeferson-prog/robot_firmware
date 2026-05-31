use crate::middleware::message::Message;

pub trait PublisherManager {
    fn publish(&self, message: &mut Message<'_>);
}