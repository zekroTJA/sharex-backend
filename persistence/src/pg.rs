use crate::{models::Image, Persistence};
use anyhow::Result;
use async_trait::async_trait;
use sqlx::{PgPool, Pool, Row};

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
impl Persistence for Postgres {
    async fn insert_image(&self, image: &Image) -> Result<()> {
        sqlx::query(
            r#"INSERT INTO "images" ("id", "creator_id", "created_at") 
            VALUES ($1, $2, $3)"#,
        )
        .bind(&image.id)
        .bind(&image.creator_id)
        .bind(image.created_at)
        .execute(&self.pool)
        .await?;
        Ok(())
    }

    async fn list_images(
        &self,
        user_id: &str,
        limit: Option<i64>,
        offset: Option<i64>,
    ) -> Result<Vec<Image>> {
        let limit = limit.unwrap_or(50);
        let offset = offset.unwrap_or(0);

        let rows = sqlx::query(
            r#"SELECT "id", "created_at"
            FROM "images" 
            WHERE "creator_id" = $1
            ORDER BY "created_at" DESC
            LIMIT $2
            OFFSET $3"#,
        )
        .bind(user_id)
        .bind(limit)
        .bind(offset)
        .fetch_all(&self.pool)
        .await?;

        let mut res = Vec::with_capacity(rows.len());
        for row in rows {
            let img = Image {
                creator_id: user_id.into(),
                id: row.try_get("id")?,
                created_at: row.try_get("created_at")?,
            };
            res.push(img);
        }

        Ok(res)
    }

    async fn get_image(&self, image_id: &str, user_id: Option<&str>) -> Result<Option<Image>> {
        let mut query =
            r#"SELECT "creator_id", "created_at" FROM "images" WHERE "id" = $1"#.to_string();

        if user_id.is_some() {
            query.push_str(r#" AND "user_id" = $2"#)
        }

        let mut query_builder = sqlx::query(&query).bind(image_id);
        if let Some(user_id) = user_id {
            query_builder = query_builder.bind(user_id);
        }

        let res = query_builder.fetch_optional(&self.pool).await?;

        match res {
            Some(v) => Ok(Some(Image {
                id: image_id.into(),
                creator_id: v.try_get("creator_id")?,
                created_at: v.try_get("created_at")?,
            })),
            None => Ok(None),
        }
    }

    async fn delete_image(&self, image_id: &str) -> Result<()> {
        sqlx::query(r#"DELETE FROM "images" WHERE "id" = $1"#)
            .bind(image_id)
            .execute(&self.pool)
            .await?;
        Ok(())
    }
}
