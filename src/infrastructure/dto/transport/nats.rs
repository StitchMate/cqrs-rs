use std::collections::HashMap;
use std::fmt::Debug;

use chrono::{serde::ts_seconds, DateTime, Utc};
use serde::{Deserialize, Serialize};

use crate::domain::entity::{aggregate::Aggregate, event::EventEnvelope};

#[derive(Serialize, Deserialize, Debug)]
pub struct NATSEventEnvelope<A> {
    pub aggregate_id: String,
    pub aggregate_type: String,
    pub sequence: String,
    pub payload: A,
    pub metadata: HashMap<String, String>,
    #[serde(with = "ts_seconds")]
    pub timestamp: DateTime<Utc>,
}

impl<A: Default + Serialize + Debug + Into<B::Event>, B: Aggregate> Into<EventEnvelope<B>>
    for NATSEventEnvelope<A>
{
    fn into(self) -> EventEnvelope<B> {
        return EventEnvelope {
            aggregate_id: self.aggregate_id,
            aggregate_type: self.aggregate_type,
            sequence: self.sequence,
            payload: self.payload.into(),
            metadata: self.metadata.into(),
            timestamp: self.timestamp,
        };
    }
}

impl<A: Serialize> Into<String> for NATSEventEnvelope<A> {
    fn into(self) -> String {
        return serde_json::to_string(&self).unwrap();
    }
}

impl<A: Default + Debug + Into<B::Event> + From<B::Event> + Serialize, B: Aggregate>
    From<EventEnvelope<B>> for NATSEventEnvelope<A>
{
    fn from(value: EventEnvelope<B>) -> Self {
        return Self {
            aggregate_id: value.aggregate_id,
            aggregate_type: value.aggregate_type,
            sequence: value.sequence,
            payload: value.payload.into(),
            metadata: value.metadata.into(),
            timestamp: value.timestamp,
        };
    }
}
