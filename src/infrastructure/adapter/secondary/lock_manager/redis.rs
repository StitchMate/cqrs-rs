use async_trait::async_trait;
use bb8::Pool;
use bb8_redis_cluster::RedisConnectionManager;
use redis_cluster_async::redis::AsyncCommands;
use std::{sync::Arc, time::Duration};
use tokio::time::sleep;

use anyhow::{anyhow, Result};
use once_cell::sync::OnceCell;

use crate::application::port::outbound::lock_manager::LockManager;

#[derive(Debug)]
pub struct RedisLockManager {
    pool: Pool<RedisConnectionManager>,
}

static INSTANCE: OnceCell<Arc<RedisLockManager>> = OnceCell::new();

impl RedisLockManager {
    pub async fn new(address: String) -> Result<Arc<Self>> {
        match INSTANCE.get() {
            Some(x) => return Ok(x.clone()),
            None => match RedisConnectionManager::new(vec![address]) {
                Ok(x) => {
                    let pool = Pool::builder().max_size(15).build(x).await;
                    if pool.is_err() {
                        return Err(pool.unwrap_err().into());
                    }
                    let ret = Arc::new(Self {
                        pool: pool.unwrap(),
                    });
                    INSTANCE.set(ret.clone()).expect("failed to set singleton");
                    return Ok(ret);
                }
                Err(e) => return Err(e.into()),
            },
        }
    }
}

#[async_trait]
impl LockManager for RedisLockManager {
    async fn lock(
        &self,
        aggregate_id: String,
        ttl: Option<i32>,
        attempt: Option<i32>,
        timeout: Option<Duration>,
    ) -> Result<(), anyhow::Error> {
        let mut conn = self.pool.get().await?;
        let exists: bool = conn.exists(aggregate_id.clone()).await?;

        if exists {
            if attempt == Some(3) {
                return Err(anyhow!("Failed to acquire lock"));
            }
            sleep(timeout.unwrap_or(Duration::from_secs(5))).await;
            return self
                .lock(
                    aggregate_id,
                    ttl,
                    attempt.map_or(Some(1), |v| Some(v + 1)),
                    timeout,
                )
                .await;
        }
        let _: () = conn
            .set_ex(aggregate_id, true, ttl.unwrap_or(5) as usize)
            .await?;

        return Ok(());
    }
    async fn unlock(&self, aggregate_id: String) -> Result<(), anyhow::Error> {
        let mut conn = self.pool.get().await?;
        let _: () = conn.del(aggregate_id).await?;
        return Ok(());
    }
}
