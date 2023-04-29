use crate::{errors::StatusError, Result};
use async_trait::async_trait;
use s3::{request::ResponseData, Bucket};

pub use s3::{creds::Credentials, Region};

use crate::StorageDriver;

pub struct S3Driver {
    region: Region,
    credentials: Credentials,
}

impl S3Driver {
    pub fn new(region: Region, credentials: Credentials) -> Self {
        Self {
            region,
            credentials,
        }
    }

    fn get_bucket(&self, bucket: &str) -> Result<Bucket> {
        Ok(Bucket::new(bucket, self.region.clone(), self.credentials.clone())?.with_path_style())
    }
}

#[async_trait]
impl StorageDriver for S3Driver {
    async fn put_object(
        &self,
        bucket: &str,
        path: &str,
        data: &[u8],
        content_type: &str,
    ) -> Result<()> {
        let bucket = self.get_bucket(bucket)?;
        let res = bucket
            .put_object_with_content_type(path, data, content_type)
            .await?;
        check_response(&res)?;
        Ok(())
    }

    async fn get_object(&self, bucket: &str, path: &str) -> Result<(Vec<u8>, Option<String>)> {
        let bucket = self.get_bucket(bucket)?;
        let res = bucket.get_object(path).await?;
        check_response(&res)?;
        let content_type = res.headers().get("content-type").map(|v| v.to_owned());
        Ok((res.to_vec(), content_type))
    }

    async fn delete_object(&self, bucket: &str, path: &str) -> Result<()> {
        let bucket = self.get_bucket(bucket)?;
        let res = bucket.delete_object(path).await?;
        check_response(&res)?;
        Ok(())
    }
}

fn check_response(res: &ResponseData) -> Result<()> {
    let status = res.status_code();
    if status >= 400 {
        return Err(StatusError::from(status).into());
    }

    Ok(())
}
