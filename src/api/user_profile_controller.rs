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
    models::user_profile::{NewUserProfile, UpdateUserProfile, UserProfileResponse},
    repository::user_profile_repo::{
        create_profile_sql, delete_profile_sql, get_user_profile_sql, update_profile_sql,
    },
    AppState,
};

pub async fn create_user_profile(
    State(data): State<Arc<AppState>>,
    Json(new_user_profile): Json<NewUserProfile>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    let result = create_profile_sql(data.db.clone(), new_user_profile).await;

    match result {
        Ok(user) => {
            let user_response =
                success_response("User created successfully".to_string(), json!(user));
            return Ok(user_response);
        }
        Err(e) => {
            if e.to_string()
                .contains("duplicate key value violates unique constraint")
            {
                let error_response = error_response(
                    "fail".to_string(),
                    "Note with that title already exists".to_string(),
                );
                return Err(error_response);
            }
            let error_response = error_response(
                StatusCode::INTERNAL_SERVER_ERROR.to_string(),
                format!("{:?}", e),
            );
            return Err(error_response);
        }
    }
}

pub async fn get_user_profile(
    State(data): State<Arc<AppState>>,
    Path(username): Path<String>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    let query_result = get_user_profile_sql(data.db.clone(), username).await;
    let user_profile: UserProfileResponse;
    match query_result {
        Ok(user) => {
            user_profile = user;
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
        "User profile retrieved successfully".to_string(),
        json!(user_profile),
    ))
}

pub async fn update_user_profile(
    State(data): State<Arc<AppState>>,
    Json(update): Json<UpdateUserProfile>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    let result = update_profile_sql(data.db.clone(), update).await;
    match result {
        Ok(user) => {
            let user_response =
                success_response("User updated successfully".to_string(), json!(user));
            return Ok(user_response);
        }
        Err(e) => {
            if e.to_string()
                .contains("duplicate key value violates unique constraint")
            {
                let error_response = error_response(
                    "fail".to_string(),
                    "Note with that title already exists".to_string(),
                );
                return Err(error_response);
            }
            let error_response = error_response(
                StatusCode::INTERNAL_SERVER_ERROR.to_string(),
                format!("{:?}", e),
            );
            return Err(error_response);
        }
    }
}

pub async fn delete_user_profile(
    Path(username): Path<String>,
    State(data): State<Arc<AppState>>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    let rows_affected = delete_profile_sql(data.db.clone(), username.clone()).await;

    if rows_affected == 0 {
        let error_response = error_response(
            "fail".to_string(),
            format!("Note with username: {} not found", username).to_string(),
        );
        return Err(error_response);
    }

    Ok(StatusCode::NO_CONTENT)
}
