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
    models::workout_log::{NewWorkoutLog, WorkoutLog, UpdateWorkoutLog},
    repository::workout_log_repo::{create_workout_log_sql, get_workout_log_sql, list_workout_log_sql, update_workout_log_sql, delete_workout_log_sql},
    AppState,
};

pub async fn create_workout_log(
    State(data): State<Arc<AppState>>,
    Json(new_entry): Json<NewWorkoutLog>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    let result = create_workout_log_sql(data.db.clone(), new_entry).await;

    match result {
        Ok(entry) => {
            let entry_response =
                success_response("Workout Log created successfully".to_string(), json!(entry));
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

pub async fn get_workout_log(
    State(data): State<Arc<AppState>>,
    Path(id): Path<uuid::Uuid>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    let query_result = get_workout_log_sql(data.db.clone(), id).await;
    let plan_workout: WorkoutLog;
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
        "Workout Log retrieved successfully".to_string(),
        json!(plan_workout),
    ))
}

pub async fn list_workout_logs(
    State(data): State<Arc<AppState>>,
    Query((username, plan_id)): Query<(String, uuid::Uuid)>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    let query_result = list_workout_log_sql(data.db.clone(), username, plan_id).await;
    let workout_logs: Vec<WorkoutLog>;
    match query_result {
        Ok(logs) => {
            workout_logs = logs;
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
        "Workout logs retrieved successfully".to_string(),
        json!(workout_logs),
    ))
}

pub async fn update_workout_log(
    State(data): State<Arc<AppState>>,
    Json(update): Json<UpdateWorkoutLog>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    let result = update_workout_log_sql(data.db.clone(), update).await;
    match result {
        Ok(entry) => {
            let log_response = success_response(
                "Workout Log updated successfully".to_string(),
                json!(entry),
            );
            return Ok(log_response);
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

pub async fn delete_workout_log(
    State(data): State<Arc<AppState>>,
    Path(id): Path<uuid::Uuid>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    let rows_affected = delete_workout_log_sql(data.db.clone(), id).await;

    if rows_affected == 0 {
        let error_response = error_response(
            "fail".to_string(),
            format!("Workout Log with id: {} not found", id).to_string(),
        );
        return Err(error_response);
    }

    Ok(StatusCode::NO_CONTENT)
}
