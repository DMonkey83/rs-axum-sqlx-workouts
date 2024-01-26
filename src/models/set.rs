use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;


#[derive(Debug, FromRow, Serialize,  Deserialize)]
pub struct Set { 
    pub id: i64,
    pub exercise_log_id: i64,
    pub set_number: i8,
    pub rest_duration: String,
    pub res_completed: i8,
    pub notes: String,
    pub created_at: Option<DateTime<Utc>>,
}

#[derive(Debug, FromRow, Serialize, Deserialize)]
pub struct NewSet { 
    pub exercise_log_id: i64,
    pub set_number: i8,
    pub rest_duration: String,
    pub res_completed: i8,
    pub notes: String,
}

#[derive(Debug, FromRow, Serialize, Deserialize)]
pub struct UpdateSet { 
    pub exercise_log_id: i64,
    pub set_number: Option<i8>,
    pub rest_duration: Option<String>,
    pub res_completed: Option<i8>,
    pub notes: Option<String>,
}

