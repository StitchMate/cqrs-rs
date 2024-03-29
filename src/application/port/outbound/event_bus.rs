use async_trait::async_trait;
use futures::stream::BoxStream;

#[async_trait]
pub trait EventBus<IE, T, Q, OE>
where
    IE: Into<T>,
    T: Into<OE> + Into<Q>,
{
    async fn send_event(&self, event: IE) -> Result<(), anyhow::Error>;
    async fn receive_events(&self) -> Result<BoxStream<'_, OE>, anyhow::Error>;
}
