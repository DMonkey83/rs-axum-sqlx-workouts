use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, FromRow, Serialize, Deserialize)]
pub struct UserRole {
    pub id: uuid::Uuid,
    pub role_id: uuid::Uuid,
    pub username: String,
    pub created_at: Option<DateTime<Utc>>,
}

#[derive(Debug, FromRow, Serialize, Deserialize)]
pub struct NewUserRole {
    pub role_id: uuid::Uuid,
    pub username: String,
}
