use crate::{
    errs::{ErrorKind, Result},
    Controller,
};
use envconfig::Envconfig;
use persistence::implementations::pg::Postgres;
use std::str::FromStr;
use storage::s3driver::{Credentials, Region, S3Driver};

#[derive(Debug, Envconfig)]
struct GeneralEnvConfig {
    #[envconfig(from = "STORAGE_BUCKET", default = "imgupload")]
    pub storage_bucket: String,
}

#[derive(Debug, Envconfig)]
struct S3EnvConfig {
    #[envconfig(from = "STORAGE_S3_REGION")]
    pub region: String,
    #[envconfig(from = "STORAGE_S3_ENDPOINT")]
    pub endpoint: Option<String>,
    #[envconfig(from = "STORAGE_S3_ACCESS_KEY")]
    pub access_key: Option<String>,
    #[envconfig(from = "STORAGE_S3_SECRET_KEY")]
    pub secret_key: Option<String>,
    #[envconfig(from = "STORAGE_S3_SECURITY_TOKEN")]
    pub security_token: Option<String>,
    #[envconfig(from = "STORAGE_S3_SESSION_TOKEN")]
    pub session_token: Option<String>,
}

#[derive(Debug, Envconfig)]
struct DatabaseEnvConfig {
    #[envconfig(from = "PERSISTENCE_URL")]
    pub persistence_url: String,
}

impl Controller {
    pub async fn from_env() -> Result<Self> {
        let s3_config = S3EnvConfig::init_from_env()?;

        let region = match s3_config.endpoint {
            Some(endpoint) => Region::Custom {
                region: s3_config.region,
                endpoint,
            },
            None => Region::from_str(&s3_config.region)
                .map_err(|_| ErrorKind::Misconfiguration("invalid AWS S3 region"))?,
        };

        let credentials = Credentials {
            access_key: s3_config.access_key,
            secret_key: s3_config.secret_key,
            security_token: s3_config.security_token,
            session_token: s3_config.session_token,
            expiration: None,
        };

        let database_config = DatabaseEnvConfig::init_from_env()?;
        let persistence = Postgres::new(&database_config.persistence_url).await?;

        let general_config = GeneralEnvConfig::init_from_env()?;

        Ok(Controller {
            storage_bucket: general_config.storage_bucket,
            storage: Box::new(S3Driver::new(region, credentials)),
            persistence: Box::new(persistence),
        })
    }
}
