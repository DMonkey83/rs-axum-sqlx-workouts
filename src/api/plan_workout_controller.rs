use std::sync::Arc;

use axum::{
    extract::{Path, State, Query},
    http::StatusCode,
    response::IntoResponse,
    Json,
};
use serde_json::json;

use crate::{
    helpers::response::{error_response, success_response},
    AppState, models::plan_workout::{NewPlanWorkout, PlanWorkoutResponse, UpdatePlanWorkout}, repository::plan_workout_repo::{create_plan_workout_sql, get_plan_workout_sql, list_plan_workouts_sql, update_plan_workout_sql, delete_plan_workout_sql},
};

pub async fn create_plan_workout(
    State(data): State<Arc<AppState>>,
    Json(new_entry): Json<NewPlanWorkout>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    let result = create_plan_workout_sql(data.db.clone(), new_entry).await;

    match result {
        Ok(entry) => {
            let entry_response = success_response(
                "Plan Workout created successfully".to_string(),
                json!(entry),
            );
            return Ok(entry_response);
        }
        Err(e) => {
            let error_response = error_response(
                StatusCode::INTERNAL_SERVER_ERROR.to_string(),
                format!("{:?}", e),
            );
            return Err(error_response);
        }
    }
}

pub async fn get_plan_workout(
    State(data): State<Arc<AppState>>,
    Path(id): Path<uuid::Uuid>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    let query_result = get_plan_workout_sql(data.db.clone(), id).await;
    let plan_workout: PlanWorkoutResponse;
    match query_result {
        Ok(ex) => {
            plan_workout = ex;
        }
        Err(e) => {
            let error_response = error_response(
                StatusCode::INTERNAL_SERVER_ERROR.to_string(),
                format!("{:?}", e),
            );
            return Err(error_response);
        }
    }

    Ok(success_response(
        "Plan workout retrieved successfully".to_string(),
        json!(plan_workout),
    ))
}

pub async fn list_plan_workouts(
    State(data): State<Arc<AppState>>,
    Query(plan_id): Query<uuid::Uuid>
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    let query_result = list_plan_workouts_sql(data.db.clone(), plan_id).await;
    let plan_workouts: Vec<PlanWorkoutResponse>;
    match query_result {
        Ok(user) => {
            plan_workouts = user;
        }
        Err(e) => {
            let error_response = error_response(
                StatusCode::INTERNAL_SERVER_ERROR.to_string(),
                format!("{:?}", e),
            );
            return Err(error_response);
        }
    }

    Ok(success_response(
        "Plan workouts retrieved successfully".to_string(),
        json!(plan_workouts),
    ))
}

pub async fn update_plan_workout(
    State(data): State<Arc<AppState>>,
    Json(update): Json<UpdatePlanWorkout>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    let result = update_plan_workout_sql(data.db.clone(), update).await;
    match result {
        Ok(entry) => {
            let user_response = success_response(
                "Plan Workout updated successfully".to_string(),
                json!(entry),
            );
            return Ok(user_response);
        }
        Err(e) => {
            let error_response = error_response(
                StatusCode::INTERNAL_SERVER_ERROR.to_string(),
                format!("{:?}", e),
            );
            return Err(error_response);
        }
    }
}

pub async fn delete_plan_workout(
    State(data): State<Arc<AppState>>,
    Path(id): Path<uuid::Uuid>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    let rows_affected = delete_plan_workout_sql(data.db.clone(), id).await;

    if rows_affected == 0 {
        let error_response = error_response(
            "fail".to_string(),
            format!("Plan workout with id: {} not found", id).to_string(),
        );
        return Err(error_response);
    }

    Ok(StatusCode::NO_CONTENT)
}
