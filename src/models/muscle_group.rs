use serde::{Deserialize, Serialize};
use sqlx::FromRow;

use super::enums::MuscleGroupEnum;


#[derive(Debug, FromRow, Clone, Serialize, Deserialize)]
pub struct MuscleGroup {
    pub id: uuid::Uuid,
    pub name: String,
    pub workout_id: uuid::Uuid,
    pub muscle_group: MuscleGroupEnum,
}

#[derive(Debug, FromRow, Serialize, Deserialize)]
pub struct NewMuscleGroup {
    pub name: String,
    pub workout_id: uuid::Uuid,
    pub muscle_group: MuscleGroupEnum,
}

#[derive(Debug, FromRow, Serialize, Deserialize)]
pub struct UpdateMuscleGroup {
    pub id: uuid::Uuid,
    pub name: Option<String>,
    pub muscle_group: Option<MuscleGroupEnum>,
}
