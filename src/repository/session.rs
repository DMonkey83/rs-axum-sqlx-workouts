use std::sync::Arc;

use axum::{http::StatusCode, Json};

use crate::{
    models::token::{self, TokenDetails},
    AppState,
};

use redis::AsyncCommands;

pub fn generate_token(
    user_id: uuid::Uuid,
    max_age: i64,
    private_key: String,
) -> Result<TokenDetails, (StatusCode, Json<serde_json::Value>)> {
    token::generate_jwt_token(user_id, max_age, private_key).map_err(|e| {
        let error_response = serde_json::json!({
            "status": "error",
            "message": format!("error generating token: {}", e),
        });
        (StatusCode::INTERNAL_SERVER_ERROR, Json(error_response))
    })
}

pub async fn save_token_data_to_redis(
    data: &Arc<AppState>,
    token_details: &TokenDetails,
    max_age: i64,
) -> Result<(), (StatusCode, Json<serde_json::Value>)> {
    let mut redis_client = data
        .redis_client
        .get_async_connection()
        .await
        .map_err(|e| {
            let error_response = serde_json::json!({
                "status": "error",
                "message": format!("Redis error: {}", e),
            });
            (StatusCode::INTERNAL_SERVER_ERROR, Json(error_response))
        })?;
    println!("token_details.token_uuid.to_string(): {}", token_details.token_uuid.to_string());
    redis_client
        .set_ex(
            token_details.token_uuid.to_string(),
            token_details.user_id.to_string(),
            (max_age * 60) as u64,
        )
        .await
        .map_err(|e| {
            let error_response = serde_json::json!({
                "status": "error",
                "message": format_args!("{}", e),
            });
            (StatusCode::UNPROCESSABLE_ENTITY, Json(error_response))
        })?;
    Ok(())
}
