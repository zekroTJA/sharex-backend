pub mod images;
pub mod tokens;

use crate::definitions::Persistence;
use anyhow::Result;
use async_trait::async_trait;
use sqlx::{PgPool, Pool};

pub struct Postgres {
    pool: Pool<sqlx::Postgres>,
}

impl Postgres {
    pub async fn new(conn_str: &str) -> Result<Self> {
        let pool = PgPool::connect(conn_str).await?;
        Ok(Self { pool })
    }
}

#[async_trait]
impl Persistence for Postgres {}
