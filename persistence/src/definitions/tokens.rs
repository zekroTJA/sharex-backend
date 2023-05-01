use async_trait::async_trait;

use crate::models::Token;
use anyhow::Result;

#[async_trait]
pub trait Tokens {
    async fn create_token(&self, token: &Token) -> Result<()>;
    async fn get_tokens(&self, user_id: &str) -> Result<Vec<Token>>;
    async fn get_token_by_hash(&self, hash: &str) -> Result<Option<Token>>;
    async fn delete_token(&self, id: &str) -> Result<()>;
}
