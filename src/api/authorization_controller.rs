use std::sync::Arc;

use argon2::{PasswordHash, Argon2, PasswordVerifier};
use axum::{extract::State, Json, response::Response, response::IntoResponse, http::{StatusCode, HeaderMap, header}, Extension};
use axum_extra::extract::{cookie::{Cookie, SameSite}, CookieJar};
use redis::AsyncCommands;
use serde_json::json;

use crate::{
    AppState, 
    models::{user::{NewUser, User}, token},
    models::credentials::Credentials, repository::session::{generate_token, save_token_data_to_redis}, auth::JWTAuthMiddleware,
};

use super::hash_password;

pub async fn register_user_handler(
    State(data): State<Arc<AppState>>,
    Json(body): Json<NewUser>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    let user_exists: Option<bool> =
        sqlx::query_scalar("SELECT EXISTS(SELECT 1 FROM users WHERE username = $1)")
            .bind(body.username.to_owned().to_ascii_lowercase())
            .fetch_one(&data.db)
            .await
            .map_err(|e| {
                let error_response = serde_json::json!({
                    "status": "fail",
                    "message": format!("Database error: {}", e),
                });
                (StatusCode::INTERNAL_SERVER_ERROR, Json(error_response))
            })?;

    if let Some(exists) = user_exists {
        if exists {
            let error_response = serde_json::json!({
                "status": "fail",
                "message": "User with that email already exists",
            });
            return Err((StatusCode::CONFLICT, Json(error_response)));
        }
    }

    let hashed_password = hash_password(body.password_hash);

    let user = sqlx::query_as!(
        User,
        "INSERT INTO users (username,password_hash) VALUES ($1, $2) RETURNING *",
        body.username.to_string(),
        hashed_password.unwrap()
    )
    .fetch_one(&data.db)
    .await
    .map_err(|e| {
        let error_response = serde_json::json!({
            "status": "fail",
            "message": format!("Database error: {}", e),
        });
        (StatusCode::INTERNAL_SERVER_ERROR, Json(error_response))
    })?;

    let user_response = serde_json::json!({"status": "success","data": serde_json::json!({
        "user": &user
    })});

    Ok(Json(user_response))
}

pub async fn login_user_handler(
    State(data): State<Arc<AppState>>,
    Json(body): Json<Credentials>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    let user = sqlx::query_as!(
        User,
        "SELECT * FROM users WHERE username = $1",
        body.username.to_ascii_lowercase()
    )
    .fetch_optional(&data.db)
    .await
    .map_err(|e| {
        let error_response = serde_json::json!({
            "status": "error",
            "message": format!("Database error: {}", e),
        });
        (StatusCode::INTERNAL_SERVER_ERROR, Json(error_response))
    })?
    .ok_or_else(|| {
        let error_response = serde_json::json!({
            "status": "fail",
            "message": "Invalid username or password",
        });
        (StatusCode::BAD_REQUEST, Json(error_response))
    })?;

    let is_valid = match PasswordHash::new(&user.password_hash) {
        Ok(parsed_hash) => Argon2::default()
            .verify_password(body.password.as_bytes(), &parsed_hash)
            .map_or(false, |_| true),
        Err(_) => false,
    };

    if !is_valid {
        let error_response = serde_json::json!({
            "status": "fail",
            "message": "Invalid email or password"
        });
        return Err((StatusCode::BAD_REQUEST, Json(error_response)));
    }

    let access_token_details = generate_token(
        user.id,
        data.env.access_token_max_age,
        data.env.access_token_private_key.to_owned(),
    )?;
    let refresh_token_details = generate_token(
        user.id,
        data.env.refresh_token_max_age,
        data.env.refresh_token_private_key.to_owned(),
    )?;

    save_token_data_to_redis(&data, &access_token_details, data.env.access_token_max_age).await?;
    save_token_data_to_redis(
        &data,
        &refresh_token_details,
        data.env.refresh_token_max_age,
    )
    .await?;

    let access_cookie = Cookie::build(
        ("access_token",
        access_token_details.token.clone().unwrap_or_default()),
    )
    .path("/")
    .max_age(time::Duration::minutes(data.env.access_token_max_age * 60))
    .same_site(SameSite::Lax)
    .http_only(true);

    let refresh_cookie = Cookie::build(
        ("refresh_token",
        refresh_token_details.token.unwrap_or_default()),
    )
    .path("/")
    .max_age(time::Duration::minutes(data.env.refresh_token_max_age * 60))
    .same_site(SameSite::Lax)
    .http_only(true);

    let logged_in_cookie = Cookie::build(("logged_in", "true"))
        .path("/")
        .max_age(time::Duration::minutes(data.env.access_token_max_age * 60))
        .same_site(SameSite::Lax)
        .http_only(false);

    let mut response = Response::new(
        json!({"status": "success", "access_token": access_token_details.token.unwrap()})
            .to_string(),
    );
    let mut headers = HeaderMap::new();
    headers.append(
        header::SET_COOKIE,
        access_cookie.to_string().parse().unwrap(),
    );
    headers.append(
        header::SET_COOKIE,
        refresh_cookie.to_string().parse().unwrap(),
    );
    headers.append(
        header::SET_COOKIE,
        logged_in_cookie.to_string().parse().unwrap(),
    );

    response.headers_mut().extend(headers);
    Ok(response)
}

pub async fn refresh_access_token_handler(
    cookie_jar: CookieJar,
    State(data): State<Arc<AppState>>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    let message = "could not refresh access token";

    let refresh_token = cookie_jar
        .get("refresh_token")
        .map(|cookie| cookie.value().to_string())
        .ok_or_else(|| {
            let error_response = serde_json::json!({
                "status": "fail",
                "message": message
            });
            (StatusCode::FORBIDDEN, Json(error_response))
        })?;

    let refresh_token_details =
        match token::verify_jwt_token(data.env.refresh_token_public_key.to_owned(), &refresh_token)
        {
            Ok(token_details) => token_details,
            Err(e) => {
                let error_response = serde_json::json!({
                    "status": "fail",
                    "message": format_args!("{:?}", e)
                });
                return Err((StatusCode::UNAUTHORIZED, Json(error_response)));
            }
        };

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

    let redis_token_user_id = redis_client
        .get::<_, String>(refresh_token_details.token_uuid.to_string())
        .await
        .map_err(|_| {
            let error_response = serde_json::json!({
                "status": "error",
                "message": "Token is invalid or session has expired",
            });
            (StatusCode::UNAUTHORIZED, Json(error_response))
        })?;

    let user_id_uuid = uuid::Uuid::parse_str(&redis_token_user_id).map_err(|_| {
        let error_response = serde_json::json!({
            "status": "error",
            "message": "Token is invalid or session has expired",
        });
        (StatusCode::UNAUTHORIZED, Json(error_response))
    })?;

    let user = sqlx::query_as!(User, "SELECT * FROM users WHERE id = $1", user_id_uuid)
        .fetch_optional(&data.db)
        .await
        .map_err(|e| {
            let error_response = serde_json::json!({
                "status": "fail",
                "message": format!("Error fetching user from database: {}", e),
            });
            (StatusCode::INTERNAL_SERVER_ERROR, Json(error_response))
        })?;

    let user = user.ok_or_else(|| {
        let error_response = serde_json::json!({
            "status": "fail",
            "message": "The user belonging to this token no longer exists".to_string(),
        });
        (StatusCode::UNAUTHORIZED, Json(error_response))
    })?;

    let access_token_details = generate_token(
        user.id,
        data.env.access_token_max_age,
        data.env.access_token_private_key.to_owned(),
    )?;

    save_token_data_to_redis(&data, &access_token_details, data.env.access_token_max_age).await?;

    let access_cookie = Cookie::build(
        ("access_token",
        access_token_details.token.clone().unwrap_or_default()),
    )
    .path("/")
    .max_age(time::Duration::minutes(data.env.access_token_max_age * 60))
    .same_site(SameSite::Lax)
    .http_only(true);

    let logged_in_cookie = Cookie::build(("logged_in", "true"))
        .path("/")
        .max_age(time::Duration::minutes(data.env.access_token_max_age * 60))
        .same_site(SameSite::Lax)
        .http_only(false);

    let mut response = Response::new(
        json!({"status": "success", "access_token": access_token_details.token.unwrap()})
            .to_string(),
    );
    let mut headers = HeaderMap::new();
    headers.append(
        header::SET_COOKIE,
        access_cookie.to_string().parse().unwrap(),
    );
    headers.append(
        header::SET_COOKIE,
        logged_in_cookie.to_string().parse().unwrap(),
    );

    response.headers_mut().extend(headers);
    Ok(response)
}

pub async fn logout_handler(
    cookie_jar: CookieJar,
    Extension(auth_guard): Extension<JWTAuthMiddleware>,
    State(data): State<Arc<AppState>>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    let message = "Token is invalid or session has expired";

    let refresh_token = cookie_jar
        .get("refresh_token")
        .map(|cookie| cookie.value().to_string())
        .ok_or_else(|| {
            let error_response = serde_json::json!({
                "status": "fail",
                "message": message
            });
            (StatusCode::FORBIDDEN, Json(error_response))
        })?;

    let refresh_token_details =
        match token::verify_jwt_token(data.env.refresh_token_public_key.to_owned(), &refresh_token)
        {
            Ok(token_details) => token_details,
            Err(e) => {
                let error_response = serde_json::json!({
                    "status": "fail",
                    "message": format_args!("{:?}", e)
                });
                return Err((StatusCode::UNAUTHORIZED, Json(error_response)));
            }
        };

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

    redis_client
        .del(&[
            refresh_token_details.token_uuid.to_string(),
            auth_guard.access_token_uuid.to_string(),
        ])
        .await
        .map_err(|e| {
            let error_response = serde_json::json!({
                "status": "error",
                "message": format_args!("{:?}", e)
            });
            (StatusCode::INTERNAL_SERVER_ERROR, Json(error_response))
        })?;

    let access_cookie = Cookie::build(("access_token", ""))
        .path("/")
        .max_age(time::Duration::minutes(-1))
        .same_site(SameSite::Lax)
        .http_only(true);
    let refresh_cookie = Cookie::build(("refresh_token", ""))
        .path("/")
        .max_age(time::Duration::minutes(-1))
        .same_site(SameSite::Lax)
        .http_only(true);

    let logged_in_cookie = Cookie::build(("logged_in", "true"))
        .path("/")
        .max_age(time::Duration::minutes(-1))
        .same_site(SameSite::Lax)
        .http_only(false);

    let mut headers = HeaderMap::new();
    headers.append(
        header::SET_COOKIE,
        access_cookie.to_string().parse().unwrap(),
    );
    headers.append(
        header::SET_COOKIE,
        refresh_cookie.to_string().parse().unwrap(),
    );
    headers.append(
        header::SET_COOKIE,
        logged_in_cookie.to_string().parse().unwrap(),
    );

    let mut response = Response::new(json!({"status": "success"}).to_string());
    response.headers_mut().extend(headers);
    Ok(response)
}

pub async fn get_me_handler(
    Extension(jwtauth): Extension<JWTAuthMiddleware>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    let json_response = serde_json::json!({
        "status":  "success",
        "data": serde_json::json!({
            "user": &jwtauth.user
        })
    });

    Ok(Json(json_response))
}

