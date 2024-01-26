use axum::{Json, http::StatusCode};
use serde::{Deserialize, Serialize};
use serde_json::{self, Value};

#[derive(Serialize, Deserialize)]
pub struct Response {
    pub status: String,
    pub message: String,
    pub data: Value,
}

#[derive(Serialize, Deserialize)]
pub struct ErrorResponse {
    pub status: String,
    pub message: String,
}

pub fn success_response(
    message: String,
    data: Value,
) -> (StatusCode, Json<serde_json::Value>) {
    let response = serde_json::json!(Response {
        status: "success".to_string(),
        message,
        data,
    });
    (StatusCode::OK, Json(response))
}

pub fn error_response(
    status: String,
    message: String,
) -> (StatusCode, Json<serde_json::Value>) {
    let response = serde_json::json!(ErrorResponse {
        status,
        message,
    });
    (StatusCode::OK, Json(response))
}
