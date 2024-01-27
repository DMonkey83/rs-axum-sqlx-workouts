use serde::{Deserialize, Serialize};
use sqlx::FromRow;

use super::{
    enums::{EquipmentEnum, MuscleGroupEnum},
    exercise::ExerciseResponse,
};

#[derive(Debug, FromRow, Serialize, Deserialize)]
pub struct WorkoutExercise {
    pub id: uuid::Uuid,
    pub exercise_name: String,
    pub workout_id: uuid::Uuid,
    pub sets: i32,
    pub reps: String,
    pub rest_duration: String,
}

#[derive(Debug, FromRow, Serialize, Deserialize)]
pub struct NewWorkoutExercise {
    pub exercise_name: String,
    pub workout_id: uuid::Uuid,
    pub sets: i32,
    pub reps: String,
    pub rest_duration: String,
}

#[derive(Debug, FromRow, Serialize, Deserialize)]
pub struct UpdateWorkoutExercise {
    pub exercise_name: String,
    pub workout_id: uuid::Uuid,
    pub sets: Option<i32>,
    pub reps: Option<String>,
    pub rest_duration: Option<String>,
}

#[derive(Debug, FromRow, Serialize, Deserialize)]
pub struct WorkoutExerciseResponse {
    pub id: uuid::Uuid,
    pub exercise_name: String,
    pub workout_id: uuid::Uuid,
    pub sets: i32,
    pub reps: String,
    pub rest_duration: String,
    pub equipment_required: Option<EquipmentEnum>,
    pub description: Option<String>,
    pub instructions: Option<String>,
    pub muscle_group_name: Option<MuscleGroupEnum>,
    pub max_weight_goal: Option<i32>,
    pub max_rep_goal: Option<i32>,
    pub max_weight_goal_notes: Option<String>,
    pub max_rep_goal_notes: Option<String>,
}

pub fn workout_exercise_response(
    workout_exercise: WorkoutExercise,
    exercise: ExerciseResponse,
) -> WorkoutExerciseResponse {
    let response = WorkoutExerciseResponse {
        id: workout_exercise.id,
        exercise_name: exercise.exercise_name,
        workout_id: workout_exercise.workout_id,
        sets: workout_exercise.sets,
        reps: workout_exercise.reps,
        rest_duration: workout_exercise.rest_duration,
        equipment_required: exercise.equipment_required,
        description: exercise.description,
        instructions: exercise.instructions,
        muscle_group_name: exercise.muscle_group_name,
        max_weight_goal: exercise.max_weight_goal,
        max_rep_goal: exercise.max_rep_goal,
        max_weight_goal_notes: exercise.max_weight_goal_notes,
        max_rep_goal_notes: exercise.max_rep_goal_notes,
    };
    response
}
