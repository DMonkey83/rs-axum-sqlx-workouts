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
    models::workout::{NewWorkout, UpdateWorkout, Workout},
    repository::workout_repo::{
        create_workout_sql, get_workout_sql, list_workout_sql, update_workout_sql, delete_workout_sql,
    },
    AppState,
};

pub async fn create_workout(
    State(data): State<Arc<AppState>>,
    Json(new_entry): Json<NewWorkout>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    let result = create_workout_sql(data.db.clone(), new_entry).await;

    match result {
        Ok(entry) => {
            let entry_response =
                success_response("Workout created successfully".to_string(), json!(entry));
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

pub async fn get_workout(
    State(data): State<Arc<AppState>>,
    Path(name): Path<String>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    let query_result = get_workout_sql(data.db.clone(), name).await;
    let workout: Workout;
    match query_result {
        Ok(ex) => {
            workout = ex;
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
        "Workout retrieved successfully".to_string(),
        json!(workout),
    ))
}

pub async fn list_workouts(
    State(data): State<Arc<AppState>>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    let query_result = list_workout_sql(data.db.clone()).await;
    let workouts: Vec<Workout>;
    match query_result {
        Ok(user) => {
            workouts = user;
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
        "Workout retrieved successfully".to_string(),
        json!(workouts),
    ))
}

pub async fn update_workout(
    State(data): State<Arc<AppState>>,
    Json(update): Json<UpdateWorkout>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    let result = update_workout_sql(data.db.clone(), update).await;
    match result {
        Ok(entry) => {
            let user_response =
                success_response("Workout updated successfully".to_string(), json!(entry));
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

pub async fn delete_workout(
    State(data): State<Arc<AppState>>,
    Path(id): Path<uuid::Uuid>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    let rows_affected = delete_workout_sql(data.db.clone(), id.clone()).await;

    if rows_affected == 0 {
        let error_response = error_response(
            "fail".to_string(),
            format!("Workout with id: {} not found", id).to_string(),
        );
        return Err(error_response);
    }

    Ok(StatusCode::NO_CONTENT)
}
