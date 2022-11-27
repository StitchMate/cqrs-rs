use std::time::Duration;

use async_trait::async_trait;

#[async_trait]
pub trait LockManager {
    async fn lock(
        &self,
        aggregate_id: String,
        ttl: Option<i32>,
        attempt: Option<i32>,
        timeout: Option<Duration>,
    ) -> Result<(), anyhow::Error>;
    async fn unlock(&self, aggregate_id: String) -> Result<(), anyhow::Error>;
}
