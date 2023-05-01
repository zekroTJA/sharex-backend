use super::Postgres;
use crate::{definitions::tokens::Tokens, models::Token};
use anyhow::Result;
use async_trait::async_trait;
use sqlx::Row;

#[async_trait]
impl Tokens for Postgres {
    async fn create_token(&self, token: &Token) -> Result<()> {
        sqlx::query(
            r#"INSERT INTO "tokens" ("id", "user_id", "hash", "scopes", "created_at")
            VALUES ($1, $2, $3, $4, $5)"#,
        )
        .bind(&token.id)
        .bind(&token.user_id)
        .bind(&token.hash)
        .bind(&token.scopes)
        .bind(token.created_at)
        .execute(&self.pool)
        .await?;
        Ok(())
    }

    async fn get_tokens(&self, user_id: &str) -> Result<Vec<Token>> {
        let rows = sqlx::query(
            r#"SELECT "id", "hash", "scopes", "created_at"
            FROM "tokens" 
            WHERE "user_id" = $1"#,
        )
        .bind(user_id)
        .fetch_all(&self.pool)
        .await?;

        let mut res = Vec::with_capacity(rows.len());
        for row in rows {
            let img = Token {
                user_id: user_id.into(),
                id: row.try_get("id")?,
                hash: row.try_get("hash")?,
                scopes: row.try_get("scopes")?,
                created_at: row.try_get("created_at")?,
            };
            res.push(img);
        }

        Ok(res)
    }

    async fn get_token_by_hash(&self, hash: &str) -> Result<Option<Token>> {
        let row = sqlx::query(
            r#"SELECT ("id", "user_id", "scopes", "created_at")
            FROM "tokens"
            WHERE "hash" = $1"#,
        )
        .bind(hash)
        .fetch_optional(&self.pool)
        .await?;

        let Some(row) = row else {
            return Ok(None);
        };

        let img = Token {
            hash: hash.into(),
            user_id: row.try_get("user_id")?,
            id: row.try_get("id")?,
            scopes: row.try_get("scopes")?,
            created_at: row.try_get("created_at")?,
        };

        Ok(Some(img))
    }

    async fn delete_token(&self, id: &str) -> Result<()> {
        sqlx::query(r#"DELETE FROM "tokens" WHERE "id" = $1"#)
            .bind(id)
            .execute(&self.pool)
            .await?;
        Ok(())
    }
}
