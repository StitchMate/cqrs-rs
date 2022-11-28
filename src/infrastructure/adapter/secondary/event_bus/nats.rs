use std::sync::Arc;

use async_trait::async_trait;
use futures::{stream::BoxStream, Stream, StreamExt};
use serde::de::DeserializeOwned;
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
impl<T: Sync + Send + 'static, A: Into<T> + From<T> + Into<String> + DeserializeOwned>
    EventBus<T, A, String, T> for NATSBus
{
    async fn send_event(&self, event: T) -> Result<(), anyhow::Error> {
        let client = self.connection.clone();
        let evt: A = event.into();
        let transport_evt: String = evt.into();
        match client.publish("test", transport_evt) {
            Ok(_) => Ok(()),
            Err(e) => Err(e.into()),
        }
    }

    async fn receive_events(&self) -> Result<BoxStream<'_, T>, anyhow::Error> {
        let client = self.connection.clone();
        let sub = match client.subscribe("test") {
            Ok(x) => x,
            Err(e) => return Err(e.into()),
        };
        let sub_iter = sub.into_iter();
        let stream = iter(sub_iter);
        let stream = stream.map(|x| {
            let event: A = serde_json::from_slice(&x.data).unwrap();
            return event.into();
        });
        return Ok(stream.boxed());
    }
}
