use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;


#[derive(Debug, FromRow, Serialize,  Deserialize)]
pub struct ExerciseLog { 
    pub id: uuid::Uuid,
    pub log_id: uuid::Uuid,
    pub exercise_name: String,
    pub sets_completed: i32,
    pub repetitions_completed: i32,
    pub weight_lifted: i32,
    pub notes: String,
    pub created_at: Option<DateTime<Utc>>,
}

#[derive(Debug, FromRow, Serialize, Deserialize)]
pub struct NewExerciseLog { 
    pub log_id: uuid::Uuid,
    pub exercise_name: String,
    pub sets_completed: i32,
    pub repetitions_completed: i32,
    pub weight_lifted: i32,
    pub notes: String,
}

#[derive(Debug, FromRow, Serialize, Deserialize)]
pub struct UpdateExerciseLog { 
    pub log_id: uuid::Uuid,
    pub sets_completed: Option<i32>,
    pub repetitions_completed: Option<i32>,
    pub weight_lifted: Option<i32>,
    pub notes: Option<String>,
}

