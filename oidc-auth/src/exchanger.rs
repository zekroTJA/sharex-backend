use envconfig::Envconfig;

use crate::errs::Result;
use crate::{models::*, Validator};

#[derive(Debug, Envconfig)]
pub struct Exchanger {
    #[envconfig(from = "AUTH_BASEURL")]
    base_url: String,
    #[envconfig(from = "AUTH_CLIENTID")]
    client_id: String,
    #[envconfig(from = "AUTH_CLIENTSECRET")]
    client_secret: String,
    #[envconfig(from = "AUTH_REDIRECTURI")]
    redirect_uri: String,
}

impl Exchanger {
    pub fn new<T: Into<String>>(
        base_url: T,
        client_id: T,
        client_secret: T,
        redirect_uri: T,
    ) -> Self {
        Self {
            base_url: base_url.into(),
            client_id: client_id.into(),
            client_secret: client_secret.into(),
            redirect_uri: redirect_uri.into(),
        }
    }

    pub fn from_env() -> Result<Self> {
        Ok(Self::init_from_env()?)
    }

    pub fn get_validator(&self) -> Validator {
        Validator::new(&self.base_url)
    }

    pub fn get_redirect_url(&self, scopes: &[&str]) -> String {
        format!(
            "{}/authorize?response_type=code&client_id={}&redirect_uri={}&scope={}",
            &self.base_url,
            &self.client_id,
            &self.redirect_uri,
            scopes.join(" ")
        )
    }

    pub async fn exchange_code_for_token(&self, code: &str) -> Result<OptionalTokenResponse> {
        let token_data = [
            ("grant_type", "authorization_code"),
            ("client_id", &self.client_id),
            ("client_secret", &self.client_secret),
            ("code", code),
            ("redirect_uri", &self.redirect_uri),
        ];

        let res = reqwest::Client::default()
            .post(format!("{}/oauth/token", &self.base_url))
            .form(&token_data)
            .send()
            .await?
            .error_for_status()?;

        Ok(res.json().await?)
    }
}
