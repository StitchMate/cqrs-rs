use std::sync::Arc;

use anyhow::Result;
use once_cell::sync::OnceCell;
use sqlx::PgPool;

#[derive(Debug)]
pub struct PostgresConnector {
    pub pool: PgPool,
}

static INSTANCE: OnceCell<Arc<PostgresConnector>> = OnceCell::new();

impl PostgresConnector {
    pub async fn new(pool: Result<PgPool>) -> Result<Arc<Self>> {
        match INSTANCE.get() {
            Some(x) => return Ok(x.clone()),
            None => match pool {
                Ok(x) => {
                    let ret = Arc::new(Self { pool: x });
                    INSTANCE.set(ret.clone()).expect("failed to set singleton");
                    return Ok(ret);
                }
                Err(e) => return Err(e.into()),
            },
        }
    }
}
