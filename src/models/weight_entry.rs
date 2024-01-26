use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;


#[derive(Debug, FromRow, Serialize,  Deserialize)]
pub struct WeightEntry { 
    pub id: uuid::Uuid,
    pub username: String,
    pub entry_date: Option<DateTime<Utc>>,
    pub weight: i32,
    pub notes: String,
    pub created_at: Option<DateTime<Utc>>,
}

#[derive(Debug, FromRow, Serialize, Deserialize)]
pub struct NewWeightEntry { 
    pub username: String,
    pub entry_date: Option<DateTime<Utc>>,
    pub weight: i32,
    pub notes: String,
}

#[derive(Debug, FromRow, Serialize, Deserialize)]
pub struct UpdateWeightEntry { 
    pub id: uuid::Uuid,
    pub username: String,
    pub entry_date: Option<DateTime<Utc>>,
    pub weight: Option<i32>,
    pub notes: Option<String>,
}

