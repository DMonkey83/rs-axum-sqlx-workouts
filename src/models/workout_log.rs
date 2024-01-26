use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;

use super::enums::{RatingEnum, FatigueLevelEnum};

#[derive(Debug, FromRow, Serialize,  Deserialize)]
pub struct WorkoutLog { 
    pub id: uuid::Uuid,
    pub username: String,
    pub plan_id: uuid::Uuid,
    pub workout_id: uuid::Uuid,
    pub log_date: Option<DateTime<Utc>>,
    pub rating: RatingEnum,
    pub fatigue_level: FatigueLevelEnum,
    pub overall_feeling: String,
    pub comments: String,
    pub workout_duration: String,
    pub total_calories_burned: i32,
    pub total_distance: i32,
    pub total_repetitions: i32,
    pub total_sets: i32,
    pub total_weight_lifterd: i32,
    pub created_at: Option<DateTime<Utc>>,
}

#[derive(Debug, FromRow, Serialize, Deserialize)]
pub struct NewWorkoutLog { 
    pub username: String,
    pub plan_id: uuid::Uuid,
    pub workout_id: uuid::Uuid,
    pub log_date: Option<DateTime<Utc>>,
    pub rating: RatingEnum,
    pub fatigue_level: FatigueLevelEnum,
    pub overall_feeling: String,
    pub comments: String,
    pub workout_duration: String,
    pub total_calories_burned: i32,
    pub total_distance: i32,
    pub total_repetitions: i32,
    pub total_sets: i32,
    pub total_weight_lifterd: i32,
}

#[derive(Debug, FromRow, Serialize, Deserialize)]
pub struct UpdateWorkoutLog { 
    pub username: String,
    pub plan_id: uuid::Uuid,
    pub workout_id: uuid::Uuid,
    pub log_date: Option<DateTime<Utc>>,
    pub rating: Option<RatingEnum>,
    pub fatigue_level: Option<FatigueLevelEnum>,
    pub overall_feeling: Option<String>,
    pub comments: Option<String>,
    pub workout_duration: Option<String>,
    pub total_calories_burned: Option<i32>,
    pub total_distance: Option<i32>,
    pub total_repetitions: Option<i32>,
    pub total_sets: Option<i32>,
    pub total_weight_lifterd: Option<i32>,
}

