use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;

use super::enums::RoleCodeEnum;


#[derive(Debug, FromRow, Serialize, Deserialize)]
pub struct Role {
    pub id: i64,
    pub code: RoleCodeEnum,
    pub name: String,
    pub created_at: Option<DateTime<Utc>>,
}

#[derive(Debug, FromRow, Serialize, Deserialize)]
pub struct NewRole {
    pub code: RoleCodeEnum,
    pub name: String,
}

#[derive(Debug, FromRow, Serialize, Deserialize)]
pub struct UpdateRole {
    pub id: i64,
    pub code: Option<RoleCodeEnum>,
    pub name: Option<String>,
}
