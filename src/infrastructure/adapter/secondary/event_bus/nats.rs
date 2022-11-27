use std::sync::Arc;

use anyhow::anyhow;
use async_trait::async_trait;
use crossbeam_channel::unbounded;
use futures::{Stream, StreamExt};
use tokio_stream::iter;

use crate::application::port::outbound::event_bus::EventBus;

pub struct NATSBus {
    connection: Arc<nats::Connection>,
}

impl NATSBus {
    pub fn new(address: String) -> Result<Self, anyhow::Error> {
        match nats::connect(address) {
            Err(e) => return Err(e.into()),
            Ok(x) => {
                return Ok(Self {
                    connection: Arc::new(x),
                })
            }
        }
    }
}

#[async_trait]
impl<T: Sync + Send + 'static + Into<String> + From<String>> EventBus<T, String, T> for NATSBus {
    async fn send_event(&self, event: T) -> Result<(), anyhow::Error> {
        let client = self.connection.clone();
        let evt: String = event.into();
        match client.publish("test", evt) {
            Ok(_) => Ok(()),
            Err(e) => Err(e.into()),
        }
    }

    async fn receive_events(&self) -> Result<Box<dyn Stream<Item = T>>, anyhow::Error> {
        let client = self.connection.clone();
        let sub = match client.subscribe("test") {
            Ok(x) => x,
            Err(e) => return Err(e.into()),
        };
        let sub_iter = sub.into_iter();
        let stream = iter(sub_iter);
        let stream = stream.map(|x| {
            let event: T = std::str::from_utf8(&x.data).unwrap().to_string().into();
            return event;
        });
        return Ok(Box::new(stream));
    }
}
