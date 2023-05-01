use sqlx::types::chrono::{DateTime, Utc};

#[derive(Clone)]
pub struct Image {
    pub id: String,
    pub creator_id: String,
    pub created_at: DateTime<Utc>,
}

#[derive(Clone)]
pub struct Token {
    pub id: String,
    pub user_id: String,
    pub hash: String,
    pub scopes: Vec<String>,
    pub created_at: DateTime<Utc>,
}
