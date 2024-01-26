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
    models::weight_entry::{NewWeightEntry, UpdateWeightEntry},
    AppState, repository::weight_entry_repo::{create_weight_entry_sql, update_weight_entry_sql, delete_weight_entry_sql},
};

pub async fn create_weight_entry(
    State(data): State<Arc<AppState>>,
    Json(new_entry): Json<NewWeightEntry>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    let result = create_weight_entry_sql(data.db.clone(), new_entry).await;

    match result {
        Ok(entry) => {
            let entry_response =
                success_response("Weight entry created successfully".to_string(), json!(entry));
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

pub async fn update_weight_entry(
    State(data): State<Arc<AppState>>,
    Json(update): Json<UpdateWeightEntry>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    let result = update_weight_entry_sql(data.db.clone(), update).await;
    match result {
        Ok(entry) => {
            let user_response =
                success_response("Weight Entry updated successfully".to_string(), json!(entry));
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

pub async fn delete_weight_entry(
    State(data): State<Arc<AppState>>,
    Path(username): Path<String>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    let rows_affected = delete_weight_entry_sql(data.db.clone(), username.clone()).await;

    if rows_affected == 0 {
        let error_response = error_response(
            "fail".to_string(),
            format!("Weight entry with username: {} not found", username).to_string(),
        );
        return Err(error_response);
    }

    Ok(StatusCode::NO_CONTENT)
}
