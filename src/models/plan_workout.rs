use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;

use super::enums::WorkoutDayEnum;

#[derive(Debug, FromRow, Serialize, Deserialize)]
pub struct PlanWorkout {
    pub id: uuid::Uuid,
    pub plan_id: uuid::Uuid,
    pub workout_id: uuid::Uuid,
    pub workout_day: WorkoutDayEnum,
    pub notes: String,
    pub created_at: Option<DateTime<Utc>>,
}

#[derive(Debug, FromRow, Serialize, Deserialize)]
pub struct NewPlanWorkout {
    pub plan_id: uuid::Uuid,
    pub workout_id: uuid::Uuid,
    pub workout_day: WorkoutDayEnum,
    pub notes: String,
}

#[derive(Debug, FromRow, Serialize, Deserialize)]
pub struct UpdatePlanWorkout {
    pub id: uuid::Uuid,
    pub workout_day: Option<WorkoutDayEnum>,
    pub notes: Option<String>,
}

#[derive(Debug, FromRow, Serialize, Deserialize)]
pub struct PlanWorkoutResponse {
    pub id: uuid::Uuid,
    pub plan_id: uuid::Uuid,
    pub workout_id: uuid::Uuid,
    pub workout_day: WorkoutDayEnum,
    pub notes: String,
    pub workout_name: String,
    pub workout_notes: String,
    pub created_at: Option<DateTime<Utc>>,
}
