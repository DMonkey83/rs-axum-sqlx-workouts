use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;

use super::{
    enums::{GenderEnum, HeightEnum, WeightEnum},
    weight_entry::WeightEntry,
};

#[derive(Debug, FromRow, Serialize, Deserialize)]
pub struct UserProfile {
    pub id: uuid::Uuid,
    pub username: String,
    pub first_name: String,
    pub last_name: String,
    pub email: String,
    pub age: i32,
    pub gender: GenderEnum,
    pub height: i32,
    pub preferred_weight_unit: WeightEnum,
    pub preferred_height_unit: HeightEnum,
    pub created_at: Option<DateTime<Utc>>,
}

#[derive(Debug, FromRow, Serialize, Deserialize)]
pub struct NewUserProfile {
    pub username: String,
    pub first_name: String,
    pub last_name: String,
    pub email: String,
    pub age: i32,
    pub gender: GenderEnum,
    pub height: i32,
    pub preferred_weight_unit: WeightEnum,
    pub preferred_height_unit: HeightEnum,
}

#[derive(Debug, FromRow, Serialize, Deserialize)]
pub struct UpdateUserProfile {
    pub username: Option<String>,
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub email: Option<String>,
    pub age: Option<i32>,
    pub gender: Option<GenderEnum>,
    pub height: Option<i32>,
    pub preferred_weight_unit: Option<WeightEnum>,
    pub preferred_height_unit: Option<HeightEnum>,
}

#[derive(Debug, FromRow, Serialize, Deserialize)]
pub struct UserProfileResponse {
    pub username: String,
    pub first_name: String,
    pub last_name: String,
    pub email: String,
    pub age: i32,
    pub gender: GenderEnum,
    pub height: i32,
    pub preferred_weight_unit: WeightEnum,
    pub preferred_height_unit: HeightEnum,
    pub weight_entries: Vec<WeightEntry>,
    pub created_at: Option<DateTime<Utc>>,
}

pub fn user_profile_response(
    profile: UserProfile,
    weight_entries: Vec<WeightEntry>,
) -> UserProfileResponse {
    let response = UserProfileResponse {
        username: profile.username.to_string(),
        first_name: profile.first_name.to_string(),
        last_name: profile.last_name.to_string(),
        email: profile.email.to_string(),
        age: profile.age,
        gender: profile.gender,
        height: profile.height,
        preferred_weight_unit: profile.preferred_weight_unit,
        preferred_height_unit: profile.preferred_height_unit,
        weight_entries,
        created_at: profile.created_at,
    };
    response
}
