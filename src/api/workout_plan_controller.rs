use std::sync::Arc;

use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    response::IntoResponse,
    Json,
};
use serde_json::json;

use crate::{
    helpers::response::{error_response, success_response},
    models::workout_plan::{
        ListWorkoutPlanResponse, NewWorkoutPlan, UpdateWorkoutPlan, WorkoutPlanResponse,
    },
    repository::workout_plan_repo::{
        create_workout_plan_sql, delete_workout_plan_sql, get_workout_plan_sql,
        list_workout_plans_sql, update_workout_plan_sql,
    },
    AppState,
};

pub async fn create_workout_plan(
    State(data): State<Arc<AppState>>,
    Json(new_entry): Json<NewWorkoutPlan>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    let result = create_workout_plan_sql(data.db.clone(), new_entry).await;

    match result {
        Ok(entry) => {
            let entry_response = success_response(
                "Workout Plan created successfully".to_string(),
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

pub async fn get_workout_plan(
    State(data): State<Arc<AppState>>,
    Path((plan_name, username)): Path<(String, String)>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    let query_result = get_workout_plan_sql(data.db.clone(), plan_name, username).await;
    let plan_workout: WorkoutPlanResponse;
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
        "Workout Plan retrieved successfully".to_string(),
        json!(plan_workout),
    ))
}

pub async fn list_workout_plans(
    State(data): State<Arc<AppState>>,
    Query(username): Query<String>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    let query_result = list_workout_plans_sql(data.db.clone(), username).await;
    let plans: Vec<ListWorkoutPlanResponse>;
    match query_result {
        Ok(user) => {
            plans = user;
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
        "Workout plans retrieved successfully".to_string(),
        json!(plans),
    ))
}

pub async fn update_workout_plan(
    State(data): State<Arc<AppState>>,
    Json(update): Json<UpdateWorkoutPlan>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    let result = update_workout_plan_sql(data.db.clone(), update).await;
    match result {
        Ok(entry) => {
            let user_response = success_response(
                "Workout Plan updated successfully".to_string(),
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

pub async fn delete_workout_plan(
    State(data): State<Arc<AppState>>,
    Path(id): Path<uuid::Uuid>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    let rows_affected = delete_workout_plan_sql(data.db.clone(), id).await;

    if rows_affected == 0 {
        let error_response = error_response(
            "fail".to_string(),
            format!("Workout Plan with id: {} not found", id).to_string(),
        );
        return Err(error_response);
    }

    Ok(StatusCode::NO_CONTENT)
}
