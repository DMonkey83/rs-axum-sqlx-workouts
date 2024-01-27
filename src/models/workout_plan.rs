use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;

use super::{
    enums::{DifficultyEnum, VisibilityEnum, WorkoutGoalEnum},
    plan_workout::PlanWorkoutResponse,
};

#[derive(Debug, FromRow, Serialize, Deserialize)]
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

#[derive(Debug, FromRow, Serialize, Deserialize)]
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
    pub plan_workouts: Vec<PlanWorkoutResponse>,
    pub created_at: Option<DateTime<Utc>>,
}

#[derive(Debug, FromRow, Serialize, Deserialize)]
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
) -> WorkoutPlanResponse {
    let response = WorkoutPlanResponse {
        id: plan.id,
        username: plan.username,
        plan_name: plan.plan_name,
        description: plan.description,
        start_date: plan.start_date,
        end_date: plan.end_date,
        goal: plan.goal,
        difficulty: plan.difficulty,
        is_public: plan.is_public,
        plan_workouts: workouts,
        created_at: plan.created_at,
    };
    response
}
