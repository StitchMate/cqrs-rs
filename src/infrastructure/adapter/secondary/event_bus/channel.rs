use anyhow::anyhow;
use async_trait::async_trait;
use crossbeam_channel::unbounded;
use futures::Stream;
use tokio_stream::iter;

use crate::application::port::outbound::event_bus::EventBus;

pub struct ChannelBus<T> {
    receiver: crossbeam_channel::Receiver<T>,
    sender: crossbeam_channel::Sender<T>,
}

impl<T> ChannelBus<T> {
    pub fn new() -> Self {
        let (tx, rx) = unbounded();
        return Self {
            receiver: rx,
            sender: tx,
        };
    }
}

#[async_trait]
impl<T: Sync + Send + 'static> EventBus<T, T, T, T> for ChannelBus<T> {
    async fn send_event(&self, event: T) -> Result<(), anyhow::Error> {
        self.sender.try_send(event).map_err(|_e| anyhow!("Unknown"))
    }

    async fn receive_events(&self) -> Result<Box<dyn Stream<Item = T>>, anyhow::Error> {
        let rx = self.receiver.clone().into_iter();
        let stream: Box<dyn Stream<Item = T>> = Box::new(iter(rx));
        return Ok(stream);
    }
}
