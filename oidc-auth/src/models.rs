use optional_struct::{optional_struct, Applyable};
use serde::{Deserialize, Serialize};

#[optional_struct]
#[derive(Debug, Serialize, Deserialize)]
pub struct TokenResponse {
    pub access_token: String,
    pub refresh_token: String,
    pub id_token: String,
    pub token_type: String,
    pub expires_in: i64,
}

#[optional_struct]
#[derive(Debug, Serialize, Deserialize)]
pub struct TokenClaims {
    pub nickname: String,
    pub name: String,
    pub picture: String,
    pub updated_at: String,
    pub email: String,
    pub email_verified: bool,
    pub iss: String,
    pub aud: String,
    pub iat: i64,
    pub exp: i64,
    pub sub: String,
    pub sid: String,
}

#[derive(Serialize)]
pub(crate) struct UserProfileReqeust {
    pub id_token: String,
}
