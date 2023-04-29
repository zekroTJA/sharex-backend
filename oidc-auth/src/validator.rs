use crate::errs::ErrorKind;
use crate::errs::Result;
use crate::models::*;
use envconfig::Envconfig;
use jsonwebtoken::{jwk::AlgorithmParameters, *};

#[derive(Debug, Envconfig)]
pub struct Validator {
    #[envconfig(from = "AUTH_BASEURL")]
    base_url: String,
}

impl Validator {
    pub fn new<T: Into<String>>(base_url: T) -> Self {
        Self {
            base_url: base_url.into(),
        }
    }

    pub fn from_env() -> Result<Self> {
        Ok(Self::init_from_env()?)
    }

    pub async fn decode_and_verify_idtoken(&self, id_token: &str) -> Result<TokenClaims> {
        let jwks = self.get_jwks().await?;

        let header = decode_header(id_token)?;
        let kid = header.kid.ok_or(ErrorKind::TokenNoKidHeader)?;
        let key = jwks.find(&kid).ok_or(ErrorKind::TokenNoKeyInJwks)?;

        let AlgorithmParameters::RSA(rsa) = &key.algorithm else {
            return Err(ErrorKind::TokenJwkInvalidAlgorithm.into());
        };

        let decoding_key = DecodingKey::from_rsa_components(&rsa.n, &rsa.e)?;
        let mut validation = Validation::new(key.common.algorithm.unwrap());
        validation.validate_exp = false;
        let decoded_token = decode(id_token, &decoding_key, &validation)?;

        Ok(decoded_token.claims)
    }

    async fn get_jwks(&self) -> Result<jwk::JwkSet> {
        Ok(reqwest::Client::default()
            .get(format!("{}/.well-known/jwks.json", &self.base_url))
            .send()
            .await?
            .error_for_status()?
            .json()
            .await?)
    }
}
