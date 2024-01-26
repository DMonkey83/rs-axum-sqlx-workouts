use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, FromRow, Serialize, Deserialize)]
pub struct Workout {
    pub id: uuid::Uuid,
    pub workout_name: String,
    pub notes: String,
    pub created_at: Option<DateTime<Utc>>,
}

#[derive(Debug, FromRow, Serialize, Deserialize)]
pub struct NewWorkout {
    pub workout_name: String,
    pub notes: String,
}

#[derive(Debug, FromRow, Serialize, Deserialize)]
pub struct UpdateWorkout {
    pub id: uuid::Uuid,
    pub workout_name: Option<String>,
    pub notes: Option<String>,
}
