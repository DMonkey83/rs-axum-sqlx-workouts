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
    models::max_weight_goal::{NewMaxWeightGoal, UpdateMaxWeightGoal},
    repository::max_weight_goal_repo::{
        create_max_weight_goal_sql, delete_max_weight_goal_sql, update_max_weight_goal_sql,
    },
    AppState,
};

pub async fn create_max_weight_goal(
    State(data): State<Arc<AppState>>,
    Json(new_entry): Json<NewMaxWeightGoal>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    let result = create_max_weight_goal_sql(data.db.clone(), new_entry).await;

    match result {
        Ok(entry) => {
            let entry_response =
                success_response("Goal created successfully".to_string(), json!(entry));
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

pub async fn update_max_weight_goal(
    State(data): State<Arc<AppState>>,
    Json(update): Json<UpdateMaxWeightGoal>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    let result = update_max_weight_goal_sql(data.db.clone(), update).await;
    match result {
        Ok(entry) => {
            let max_rep_gaol_response =
                success_response("Goal updated successfully".to_string(), json!(entry));
            return Ok(max_rep_gaol_response);
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

pub async fn delete_max_weight_goal(
    State(data): State<Arc<AppState>>,
    Path(id): Path<uuid::Uuid>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    let rows_affected = delete_max_weight_goal_sql(data.db.clone(), id.clone()).await;

    if rows_affected == 0 {
        let error_response = error_response(
            "fail".to_string(),
            format!("Goal with id: {} not found", id).to_string(),
        );
        return Err(error_response);
    }

    Ok(StatusCode::NO_CONTENT)
}
