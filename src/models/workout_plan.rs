
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;

use super::enums::{WorkoutGoalEnum, DifficultyEnum, VisibilityEnum};


#[derive(Debug, FromRow, Serialize,  Deserialize)]
pub struct WorkoutPlan {
    pub id: uuid::Uuid,
    pub username: String,
    pub plan_name: String,
    pub description: String,
    pub start_date: Option<DateTime<Utc>>,
    pub end_date: Option<DateTime<Utc>>,
    pub goal: WorkoutGoalEnum,
    pub difficulty: DifficultyEnum,
    pub is_public: VisibilityEnum,
    pub created_at: Option<DateTime<Utc>>,
}

#[derive(Debug, FromRow, Serialize, Deserialize)]
pub struct NewWorkoutPlan { 
    pub username: String,
    pub plan_name: String,
    pub description: String,
    pub start_date: Option<DateTime<Utc>>,
    pub end_date: Option<DateTime<Utc>>,
    pub goal: WorkoutGoalEnum,
    pub difficulty: DifficultyEnum,
    pub is_public: VisibilityEnum,
}

#[derive(Debug, FromRow, Serialize, Deserialize)]
pub struct UpdateWorkoutPlan { 
    pub id: uuid::Uuid,
    pub plan_name: Option<String>,
    pub description: Option<String>,
    pub start_date: Option<DateTime<Utc>>,
    pub end_date: Option<DateTime<Utc>>,
    pub goal: Option<WorkoutGoalEnum>,
    pub difficulty: Option<DifficultyEnum>,
    pub is_public: Option<VisibilityEnum>,
}

