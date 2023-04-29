use async_trait::async_trait;

pub mod errors;
pub mod s3driver;

pub type Error = anyhow::Error;
pub type Result<T> = anyhow::Result<T>;

#[async_trait]
pub trait StorageDriver {
    async fn put_object(
        &self,
        bucket: &str,
        path: &str,
        data: &[u8],
        content_type: &str,
    ) -> Result<()>;
    async fn get_object(&self, bucket: &str, path: &str) -> Result<(Vec<u8>, Option<String>)>;
    async fn delete_object(&self, bucket: &str, path: &str) -> Result<()>;
}
