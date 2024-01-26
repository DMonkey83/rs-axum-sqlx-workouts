use std::sync::Arc;

use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
    Json,
};
use serde_json::json;

use crate::{
    helpers::response::{error_response, success_response},
    models::exercise::{NewExercise, UpdateExercise, Exercise},
    repository::exercise_repo::{
        create_exercise_sql, delete_exercise_sql, get_exercise_sql, update_exercise_sql, list_exercises_sql,
    },
    AppState,
};

pub async fn create_exercise(
    State(data): State<Arc<AppState>>,
    Json(new_entry): Json<NewExercise>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    let result = create_exercise_sql(data.db.clone(), new_entry).await;

    match result {
        Ok(entry) => {
            let entry_response = success_response(
                "Exercise created successfully".to_string(),
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

pub async fn get_exercise(
    State(data): State<Arc<AppState>>,
    Path(name): Path<String>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    let query_result = get_exercise_sql(data.db.clone(), name).await;
    let exercise: Exercise;
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
        "Exercise retrieved successfully".to_string(),
        json!(exercise),
    ))
}

pub async fn list_exercises(
    State(data): State<Arc<AppState>>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    let query_result = list_exercises_sql(data.db.clone()).await;
    let exercise: Vec<Exercise>;
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
        "Exercises retrieved successfully".to_string(),
        json!(exercise),
    ))
}

pub async fn update_exercise(
    State(data): State<Arc<AppState>>,
    Json(update): Json<UpdateExercise>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    let result = update_exercise_sql(data.db.clone(), update).await;
    match result {
        Ok(entry) => {
            let user_response = success_response(
                "Exercise updated successfully".to_string(),
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

pub async fn delete_exercise(
    State(data): State<Arc<AppState>>,
    Path(name): Path<String>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    let rows_affected = delete_exercise_sql(data.db.clone(), name.clone()).await;

    if rows_affected == 0 {
        let error_response = error_response(
            "fail".to_string(),
            format!("Exercise with name: {} not found", name).to_string(),
        );
        return Err(error_response);
    }

    Ok(StatusCode::NO_CONTENT)
}
