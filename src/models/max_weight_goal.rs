use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, FromRow, Serialize, Deserialize)]
pub struct MaxWeightGoal {
    pub id: uuid::Uuid,
    pub username: String,
    pub exercise_name: String,
    pub goal_weight: i32,
    pub notes: String,
    pub created_at: Option<DateTime<Utc>>,
}

#[derive(Debug, FromRow, Serialize, Deserialize)]
pub struct NewMaxWeightGoal {
    pub username: String,
    pub exercise_name: String,
    pub goal_weight: i32,
    pub notes: String,
}

#[derive(Debug, FromRow, Serialize, Deserialize)]
pub struct UpdateMaxWeightGoal {
    pub username: String,
    pub exercise_name: String,
    pub goal_weight: Option<i32>,
    pub notes: Option<String>,
}
