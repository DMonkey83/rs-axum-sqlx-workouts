use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;


#[derive(Debug, FromRow, Serialize,  Deserialize)]
pub struct MaxRepGoal { 
    pub id: uuid::Uuid,
    pub username: String,
    pub exercise_name: String,
    pub goal_reps: i32,
    pub notes: String,
    pub created_at: Option<DateTime<Utc>>,
}

#[derive(Debug, FromRow, Serialize, Deserialize)]
pub struct NewMaxRepGoal { 
    pub username: String,
    pub exercise_name: String,
    pub goal_reps: i32,
    pub notes: String,
}

#[derive(Debug, FromRow, Serialize, Deserialize)]
pub struct UpdateMaxRepGoal { 
    pub username: String,
    pub exercise_name: String,
    pub goal_reps: Option<i32>,
    pub notes: Option<String>,
}

