use std::error::Error;

use super::event::{AggregateSnapshot, DomainEvent};
use async_trait::async_trait;

#[async_trait]
pub trait Aggregate: Default + Sync + Send {
    /// The type of Command this aggregate handles
    type Command;
    /// The type of Event this aggregate emits. Must implement DomainEvent trait
    type Event: DomainEvent;
    /// The type of Error this aggregate emits.
    type Error: Error;
    /// The external services available to this Aggregate for business logic (e.g. 3rd party APIs)
    type Services: Send + Sync;

    fn aggregate_type() -> String;

    fn aggregate_id(&self) -> Option<String>;

    async fn handle(
        &self,
        command: Self::Command,
        service: &Self::Services,
    ) -> Result<Vec<Self::Event>, Self::Error>;

    fn apply(&mut self, event: Self::Event);

    fn apply_snapshot(&mut self, snapshot: AggregateSnapshot<Self>);

    fn snapshot(&mut self) -> Option<AggregateSnapshot<Self>>;
}
