use std::collections::HashMap;
use std::fmt::Debug;

use chrono::{serde::ts_seconds, DateTime, Utc};
use serde::{Deserialize, Serialize};

use crate::domain::entity::{aggregate::Aggregate, event::EventEnvelope};

#[derive(Serialize, Deserialize, Debug)]
pub struct NATSEventEnvelope<A>
where
    A: Default,
{
    pub aggregate_id: String,
    pub aggregate_type: String,
    pub sequence: String,
    pub payload: sqlx::types::Json<A>,
    pub metadata: sqlx::types::Json<HashMap<String, String>>,
    #[serde(with = "ts_seconds")]
    pub timestamp: DateTime<Utc>,
}

impl<A: Default + Debug + Into<B::Event>, B: Aggregate> Into<EventEnvelope<B>>
    for NATSEventEnvelope<A>
{
    fn into(self) -> EventEnvelope<B> {
        return EventEnvelope {
            aggregate_id: self.aggregate_id,
            aggregate_type: self.aggregate_type,
            sequence: self.sequence,
            payload: self.payload.0.into(),
            metadata: self.metadata.0,
            timestamp: self.timestamp,
        };
    }
}
