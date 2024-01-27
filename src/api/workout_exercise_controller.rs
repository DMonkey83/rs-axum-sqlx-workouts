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
    models::workout_exercise::{
        NewWorkoutExercise, UpdateWorkoutExercise, WorkoutExercise, WorkoutExerciseResponse,
    },
    repository::workout_exercise_repo::{
        create_workout_exercise_sql, delete_workout_exercise_sql, get_workout_exercise_sql,
        list_workout_exercises_sql, update_workout_exercise_sql,
    },
    AppState,
};

pub async fn create_workout_exercise(
    State(data): State<Arc<AppState>>,
    Json(new_entry): Json<NewWorkoutExercise>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    let result = create_workout_exercise_sql(data.db.clone(), new_entry).await;

    match result {
        Ok(entry) => {
            let entry_response =
                success_response("Exercise created successfully".to_string(), json!(entry));
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

pub async fn get_workout_exercise(
    State(data): State<Arc<AppState>>,
    Path((id, username)): Path<(uuid::Uuid, String)>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    println!("name: {:?}/{:?}", id, username);
    let query_result = get_workout_exercise_sql(data.db.clone(), id, username).await;
    let exercise: WorkoutExerciseResponse;
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
        "Exercise retrieved successfully".to_string(),
        json!(exercise),
    ))
}

pub async fn list_workout_exercises(
    State(data): State<Arc<AppState>>,
    Query(workout_id): Query<uuid::Uuid>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    let query_result = list_workout_exercises_sql(data.db.clone(), workout_id).await;
    let exercise: Vec<WorkoutExercise>;
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

pub async fn update_workout_exercise(
    State(data): State<Arc<AppState>>,
    Json(update): Json<UpdateWorkoutExercise>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    let result = update_workout_exercise_sql(data.db.clone(), update).await;
    match result {
        Ok(entry) => {
            let user_response =
                success_response("Exercise updated successfully".to_string(), json!(entry));
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

pub async fn delete_workout_exercise(
    State(data): State<Arc<AppState>>,
    Path(id): Path<uuid::Uuid>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    let rows_affected = delete_workout_exercise_sql(data.db.clone(), id).await;

    if rows_affected == 0 {
        let error_response = error_response(
            "fail".to_string(),
            format!("Exercise with name: {} not found", id).to_string(),
        );
        return Err(error_response);
    }

    Ok(StatusCode::NO_CONTENT)
}
