pub mod env;
pub mod errs;
pub mod models;

use chrono::Utc;
use errs::Result;
use models::Image;
use persistence::definitions::Persistence;
use storage::{
    errors::{StatusError, StatusErrorKind},
    StorageDriver,
};

use crate::errs::ErrorKind;

pub struct Controller {
    storage_bucket: String,
    storage: Box<dyn StorageDriver>,
    persistence: Box<dyn Persistence>,
}

impl Controller {
    pub fn new<T: Into<String>>(
        storage_bucket: T,
        storage: Box<dyn StorageDriver>,
        persistence: Box<dyn Persistence>,
    ) -> Self {
        Self {
            storage_bucket: storage_bucket.into(),
            storage,
            persistence,
        }
    }

    pub async fn upload_image(
        &self,
        user_id: &str,
        data: &[u8],
        content_type: &str,
    ) -> Result<Image> {
        let img = persistence::models::Image {
            id: xid::new().to_string(),
            creator_id: user_id.into(),
            created_at: Utc::now(),
        };

        self.storage
            .put_object(&self.storage_bucket, &img.id, data, content_type)
            .await?;

        self.persistence.insert_image(&img).await?;

        Ok(img.into())
    }

    pub async fn get_image(&self, image_id: &str) -> Result<(Vec<u8>, Option<String>)> {
        let res = self
            .storage
            .get_object(&self.storage_bucket, image_id)
            .await;
        match res {
            Err(err) => {
                if let Some(status_err) = err.downcast_ref::<StatusError>() {
                    if matches!(status_err.kind(), StatusErrorKind::NotFound) {
                        return Err(ErrorKind::ImageNotFound.into());
                    }
                }
                Err(err.into())
            }
            Ok(v) => Ok(v),
        }
    }

    pub async fn list_images(
        &self,
        user_id: &str,
        limit: Option<i64>,
        offset: Option<i64>,
    ) -> Result<Vec<Image>> {
        Ok(self
            .persistence
            .list_images(
                user_id,
                Some(limit.unwrap_or(50)),
                Some(offset.unwrap_or(0)),
            )
            .await?
            .iter()
            .cloned()
            .map(Image::from)
            .collect())
    }

    pub async fn delete_image(&self, user_id: &str, image_id: &str) -> Result<()> {
        let res = self.persistence.get_image(image_id, Some(user_id)).await?;
        if res.is_none() {
            return Err(ErrorKind::ImageNotFound.into());
        }

        self.storage
            .delete_object(&self.storage_bucket, image_id)
            .await?;

        self.persistence.delete_image(image_id).await?;

        Ok(())
    }
}
