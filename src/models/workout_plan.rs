
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;

use super::{enums::{WorkoutGoalEnum, DifficultyEnum, VisibilityEnum}, plan_workout::PlanWorkout};


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

#[derive(Debug, FromRow, Serialize,  Deserialize)]
pub struct WorkoutPlanResponse {
    pub id: uuid::Uuid,
    pub username: String,
    pub plan_name: String,
    pub description: String,
    pub start_date: Option<DateTime<Utc>>,
    pub end_date: Option<DateTime<Utc>>,
    pub goal: WorkoutGoalEnum,
    pub difficulty: DifficultyEnum,
    pub is_public: VisibilityEnum,
    pub plan_workouts: Vec<PlanWorkout>,
    pub created_at: Option<DateTime<Utc>>,
}

#[derive(Debug, FromRow, Serialize,  Deserialize)]
pub struct ListWorkoutPlanResponse {
    pub id: uuid::Uuid,
    pub plan_name: String,
    pub description: String,
    pub start_date: Option<DateTime<Utc>>,
    pub end_date: Option<DateTime<Utc>>,
    pub goal: WorkoutGoalEnum,
    pub difficulty: DifficultyEnum,
    pub is_public: VisibilityEnum,
    pub created_at: Option<DateTime<Utc>>,
}

pub fn workout_plan_response(
    plan: WorkoutPlan,
    workouts: Vec<PlanWorkoutResponse>,
) -> UserProfileResponse {
    let response = UserProfileResponse {
        username: profile.username.to_string(),
        first_name: profile.first_name.to_string(),
        last_name: profile.last_name.to_string(),
        email: profile.email.to_string(),
        age: profile.age,
        gender: profile.gender,
        height: profile.height,
        preferred_weight_unit: profile.preferred_weight_unit,
        preferred_height_unit: profile.preferred_height_unit,
        weight_entries,
        created_at: profile.created_at,
    };
    response
}
