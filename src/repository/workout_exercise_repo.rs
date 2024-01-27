use sqlx::{Pool, Postgres};

use crate::models::workout_exercise::{NewWorkoutExercise, WorkoutExercise, WorkoutExerciseResponse, workout_exercise_response, UpdateWorkoutExercise};

use super::exercise_repo::get_exercise_sql;


pub async fn create_workout_exercise_sql(
    data: Pool<Postgres>,
    new_entry: NewWorkoutExercise,
) -> Result<WorkoutExercise, sqlx::Error> {
    let entry = sqlx::query_as!(
        WorkoutExercise,
        r#"
            INSERT INTO workoutexercises (exercise_name, workout_id, sets, reps, rest_duration)
            VALUES ($1, $2, $3, $4, $5)
            RETURNING id, exercise_name, workout_id, sets, reps, rest_duration
        "#,
        new_entry.exercise_name.to_string(),
        new_entry.workout_id,
        new_entry.sets,
        new_entry.reps,
        new_entry.rest_duration.to_string(),
    )
    .fetch_one(&data)
    .await?;

    Ok(entry)
}

pub async fn get_workout_exercise_sql(
    data: Pool<Postgres>,
    id: uuid::Uuid,
    username: String,
) -> Result<WorkoutExerciseResponse, sqlx::Error> {
    let entry = sqlx::query_as!(
        WorkoutExercise,
        r#"
            SELECT 
                id,
                exercise_name,
                workout_id,
                sets,
                reps,
                rest_duration
            FROM workoutexercises
            WHERE id = $1
        "#,
        id,
    )
    .fetch_one(&data)
    .await?;

    let exercise = get_exercise_sql(data, entry.exercise_name.clone(), username).await;

    Ok(workout_exercise_response(entry, exercise.unwrap()))
}

pub async fn update_workout_exercise_sql(
    data: Pool<Postgres>,
    update: UpdateWorkoutExercise,
) -> Result<WorkoutExercise, sqlx::Error> {
    let result = sqlx::query_as!(
    WorkoutExercise,
        r#"
        UPDATE workoutexercises
        SET 
            sets = CASE WHEN $1::INTEGER IS NOT NULL THEN $1 ELSE sets END,
            reps = CASE WHEN $2::VARCHAR(255) IS NOT NULL THEN $2 ELSE reps END,
            rest_duration = CASE WHEN $3::VARCHAR(255) IS NOT NULL THEN $3 ELSE rest_duration END
        WHERE exercise_name = $4 AND workout_id = $5
            RETURNING id, exercise_name, workout_id, sets, reps, rest_duration
        "#,
        update.sets,
        update.reps,
        update.rest_duration,
        update.exercise_name.to_string(),
        update.workout_id,
    )
    .fetch_one(&data)
    .await?;

    Ok(result)
}

pub async fn delete_workout_exercise_sql(data: Pool<Postgres>, id: uuid::Uuid) -> u64 {
    let rows_affected = sqlx::query!("DELETE FROM workoutexercises WHERE id = $1", id)
        .execute(&data)
        .await
        .unwrap()
        .rows_affected();

    rows_affected
}

pub async fn list_workout_exercises_sql(data: Pool<Postgres>, workout_id: uuid::Uuid) -> Result<Vec<WorkoutExercise>, sqlx::Error> {
    let result = sqlx::query_as!(
        WorkoutExercise,
        r#"
        SELECT 
                id,
                exercise_name,
                workout_id,
                sets,
                reps,
                rest_duration
        FROM workoutexercises
        WHERE workout_id = $1
        ORDER BY exercise_name ASC
        "#,
        workout_id
    )
    .fetch_all(&data)
    .await;
    let exercises: Vec<WorkoutExercise>;
    match result {
        Ok(entry) => {
            exercises = entry;
        }
        Err(_) => {
            exercises = vec![];
        }
    }
    Ok(exercises)
}
