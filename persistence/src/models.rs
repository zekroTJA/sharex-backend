use sqlx::types::chrono::{DateTime, Utc};

pub struct Image {
    pub id: String,
    pub creator_id: String,
    pub created_at: DateTime<Utc>,
}
