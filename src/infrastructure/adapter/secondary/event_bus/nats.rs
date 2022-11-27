use std::sync::Arc;

use anyhow::anyhow;
use async_trait::async_trait;
use crossbeam_channel::unbounded;
use futures::{Stream, StreamExt};
use tokio_stream::{self as stream, iter};

use crate::application::port::outbound::event_bus::EventBus;

#[derive(Clone)]
pub struct NATSBus<T> {
    receiver: crossbeam_channel::Receiver<T>,
    sender: crossbeam_channel::Sender<T>,
    connection: Arc<nats::Connection>,
}

impl<T> NATSBus<T> {
    pub fn new(address: String) -> Result<Self, anyhow::Error> {
        let (tx, rx) = unbounded();
        match nats::connect(address) {
            Err(e) => return Err(e.into()),
            Ok(x) => {
                return Ok(Self {
                    receiver: rx,
                    sender: tx,
                    connection: Arc::new(x),
                })
            }
        }
    }
}

#[async_trait]
impl<T: Sync + Send + 'static + Into<String> + From<nats::Message>> EventBus<T, T> for NATSBus<T> {
    async fn send_event(&self, event: T) -> Result<(), anyhow::Error> {
        let client = self.connection.clone();
        let evt: String = event.into();
        match client.publish("test", evt) {
            Ok(_) => Ok(()),
            Err(e) => Err(e.into()),
        }
    }

    async fn receive_events(&self) -> Box<dyn Stream<Item = T>> {
        let client = self.connection.clone();
        let sub = client.subscribe("test").unwrap();
        let sub_iter = sub.into_iter();
        let stream = iter(sub_iter);
        let stream = stream.map(|x| {
            let event: T = x.into();
            return event;
        });
        return Box::new(stream);
    }
}
