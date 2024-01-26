use serde::{Deserialize, Serialize};
use sqlx::FromRow;


#[derive(Debug, FromRow, Serialize, Deserialize)]
pub struct WorkoutExercise { 
    pub id: uuid::Uuid,
    pub exercise_name: String,
    pub workout_id: uuid::Uuid,
    pub sets: i64,
    pub reps: i64,
    pub rest_duration: String,
}

#[derive(Debug, FromRow, Serialize, Deserialize)]
pub struct NewWorkoutExercise {
    pub exercise_name: String,
    pub workout_id: uuid::Uuid,
    pub sets: i64,
    pub reps: i64,
    pub rest_duration: String,
}

#[derive(Debug, FromRow, Serialize, Deserialize)]
pub struct UpdateWorkoutExercise {
    pub id: uuid::Uuid,
    pub name: Option<String>,
    pub workout_id: Option<uuid::Uuid>,
}
