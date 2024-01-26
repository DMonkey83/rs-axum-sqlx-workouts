use axum::{
    middleware,
    routing::{delete, get, patch, post},
    Router,
};
use std::sync::Arc;

use crate::{auth::auth, AppState};

use super::{
    authorization_controller::{
        get_me_handler, login_user_handler, logout_handler, refresh_access_token_handler,
        register_user_handler,
    },
    exercise_container::{
        create_exercise, delete_exercise, get_exercise, list_exercises, update_exercise,
    },
    user_profile_controller::{
        create_user_profile, delete_user_profile, get_user_profile, update_user_profile,
    },
    weight_entry_controller::{create_weight_entry, delete_weight_entry, update_weight_entry},
};

pub fn create_router(app_state: Arc<AppState>) -> Router {
    Router::new()
        .route(
            "/api/exercises",
            post(create_exercise)
                .get(list_exercises)
                .patch(update_exercise)
                .route_layer(middleware::from_fn_with_state(app_state.clone(), auth)),
        )
        .route(
            "/api/exercises/:name",
            delete(delete_exercise)
                .get(get_exercise)
                .route_layer(middleware::from_fn_with_state(app_state.clone(), auth)),
        )
        .route(
            "/api/user_profiles",
            post(create_user_profile)
                .patch(update_user_profile)
                .route_layer(middleware::from_fn_with_state(app_state.clone(), auth)),
        )
        .route(
            "/api/user_profiles/:username",
            get(get_user_profile)
                .delete(delete_user_profile)
                .route_layer(middleware::from_fn_with_state(app_state.clone(), auth)),
        )
        .route(
            "/api/weight_entries",
            patch(update_weight_entry)
                .post(create_weight_entry)
                .route_layer(middleware::from_fn_with_state(app_state.clone(), auth)),
        )
        .route(
            "/api/weight_entries/:id",
            delete(delete_weight_entry)
                .route_layer(middleware::from_fn_with_state(app_state.clone(), auth)),
        )
        .route("/api/auth/register", post(register_user_handler))
        .route("/api/auth/login", get(login_user_handler))
        .route("/api/auth/refresh", get(refresh_access_token_handler))
        .route(
            "/api/auth/logout",
            get(logout_handler)
                .route_layer(middleware::from_fn_with_state(app_state.clone(), auth)),
        )
        .route(
            "/api/users/me",
            get(get_me_handler)
                .route_layer(middleware::from_fn_with_state(app_state.clone(), auth)),
        )
        .with_state(app_state)
}
