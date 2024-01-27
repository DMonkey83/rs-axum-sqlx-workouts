use argon2::{password_hash::SaltString, Argon2, PasswordHasher};
use axum::{http::StatusCode, Json};
use rand_core::OsRng;

pub mod authorization_controller;
pub mod exercise_container;
pub mod max_rep_goal_controller;
pub mod max_weight_goal_controller;
pub mod plan_workout_controller;
pub mod routes;
pub mod user_profile_controller;
pub mod weight_entry_controller;
pub mod workout_controller;
pub mod workout_exercise_controller;
pub mod workout_plan_controller;

pub fn hash_password(password: String) -> Result<String, String> {
    let salt = SaltString::generate(&mut OsRng);
    let argon2 = Argon2::default();
    let password_hash = argon2
        .hash_password(password.as_bytes(), &salt)
        .map_err(|e| {
            let error_response = serde_json::json!({
                "status": "fail",
                "message": format!("Error while hashing password: {}", e),
            });
            (StatusCode::INTERNAL_SERVER_ERROR, Json(error_response))
        })
        .map(|hash| hash.to_string());
    Ok(password_hash.unwrap())
}
