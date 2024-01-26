use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;

use super::enums::{EquipmentEnum, MuscleGroupEnum};


#[derive(Debug, FromRow, Serialize,  Deserialize)]
pub struct Exercise {
    pub exercise_name: String,
    pub equipment_required: EquipmentEnum,
    pub description: String,
    pub instructions: String,
    pub muscle_group_name: MuscleGroupEnum,
    pub created_at: Option<DateTime<Utc>>,
}

#[derive(Debug, FromRow, Serialize, Deserialize)]
pub struct NewExercise { 
    pub exercise_name: String,
    pub equipment_required: EquipmentEnum,
    pub description: String,
    pub instructions: String,
    pub muscle_group_name: MuscleGroupEnum,
}

#[derive(Debug, FromRow, Serialize, Deserialize)]
pub struct UpdateExercise { 
    pub exercise_name: String,
    pub equipment_required: Option<EquipmentEnum>,
    pub description: Option<String>,
    pub instructions: Option<String>,
    pub muscle_group_name: Option<MuscleGroupEnum>,
}

