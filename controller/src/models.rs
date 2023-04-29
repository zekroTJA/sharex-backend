use chrono::{DateTime, Utc};
use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct Image {
    pub id: String,
    pub creator_id: String,
    pub created_at: DateTime<Utc>,
}

impl From<persistence::models::Image> for Image {
    fn from(value: persistence::models::Image) -> Self {
        let persistence::models::Image {
            id,
            creator_id,
            created_at,
        } = value;

        Self {
            id,
            creator_id,
            created_at,
        }
    }
}
