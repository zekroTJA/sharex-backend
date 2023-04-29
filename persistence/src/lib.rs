pub mod models;
pub mod pg;

use anyhow::Result;
use async_trait::async_trait;
use models::Image;

#[async_trait]
pub trait Persistence {
    async fn insert_image(&self, image: &Image) -> Result<()>;
    async fn list_images(
        &self,
        user_id: &str,
        limit: Option<i64>,
        offset: Option<i64>,
    ) -> Result<Vec<Image>>;
    async fn get_image(&self, image_id: &str, user_id: Option<&str>) -> Result<Option<Image>>;
    async fn delete_image(&self, image_id: &str) -> Result<()>;
}
