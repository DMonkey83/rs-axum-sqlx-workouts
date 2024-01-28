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
    models::exercise_log::{ExerciseLog, NewExerciseLog, UpdateExerciseLog},
    repository::exercise_log_repo::{
        create_exercise_log_sql, delete_exercise_log_sql, get_exercise_log_sql,
        list_exercise_log_sql, update_exercise_log_sql,
    },
    AppState,
};

pub async fn create_exercise_log(
    State(data): State<Arc<AppState>>,
    Json(new_entry): Json<NewExerciseLog>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    let result = create_exercise_log_sql(data.db.clone(), new_entry).await;

    match result {
        Ok(entry) => {
            let entry_response = success_response(
                "Exercise Log created successfully".to_string(),
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

pub async fn get_exercise_log(
    State(data): State<Arc<AppState>>,
    Path(id): Path<uuid::Uuid>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    println!("name: {:?}", id);
    let query_result = get_exercise_log_sql(data.db.clone(), id).await;
    let exercise: ExerciseLog;
    match query_result {
        Ok(ex) => {
            exercise = ex;
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
        "Exercise Log retrieved successfully".to_string(),
        json!(exercise),
    ))
}

pub async fn list_exercise_logs(
    State(data): State<Arc<AppState>>,
    Query(log_id): Query<uuid::Uuid>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    let query_result = list_exercise_log_sql(data.db.clone(), log_id).await;
    let exercise: Vec<ExerciseLog>;
    match query_result {
        Ok(user) => {
            exercise = user;
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
        "Exercise Logs retrieved successfully".to_string(),
        json!(exercise),
    ))
}

pub async fn update_exercise_log(
    State(data): State<Arc<AppState>>,
    Json(update): Json<UpdateExerciseLog>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    let result = update_exercise_log_sql(data.db.clone(), update).await;
    match result {
        Ok(entry) => {
            let user_response = success_response(
                "Exercise Log updated successfully".to_string(),
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

pub async fn delete_exercise_log(
    State(data): State<Arc<AppState>>,
    Path(id): Path<uuid::Uuid>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    let rows_affected = delete_exercise_log_sql(data.db.clone(), id).await;

    if rows_affected == 0 {
        let error_response = error_response(
            "fail".to_string(),
            format!("Exercise Log with id: {} not found", id).to_string(),
        );
        return Err(error_response);
    }

    Ok(StatusCode::NO_CONTENT)
}
