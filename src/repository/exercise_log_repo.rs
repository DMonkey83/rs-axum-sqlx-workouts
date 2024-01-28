use sqlx::{Pool, Postgres};

use crate::models::{
    exercise_log::{ExerciseLog, NewExerciseLog, UpdateExerciseLog},
};

use super::plan_workout_repo::list_plan_workouts_sql;

pub async fn create_exercise_log_sql(
    data: Pool<Postgres>,
    new_entry: NewExerciseLog,
) -> Result<ExerciseLog, sqlx::Error> {
    let entry = sqlx::query_as!(
        ExerciseLog,
        r#"
            INSERT INTO exerciselog (log_id, exercise_name, sets_completed, repetitions_completed, weight_lifted, notes)
            VALUES ($1, $2, $3, $4, $5, $6)
            RETURNING id, log_id, exercise_name, sets_completed, repetitions_completed, weight_lifted, notes, created_at
        "#,
        new_entry.log_id,
        new_entry.exercise_name.to_string(),
        new_entry.sets_completed,
        new_entry.repetitions_completed,
        new_entry.weight_lifted,
        new_entry.notes.to_string(),
    )
    .fetch_one(&data)
    .await?;

    Ok(entry)
}

pub async fn get_exercise_log_sql(
    data: Pool<Postgres>,
    id: uuid::Uuid,
) -> Result<ExerciseLog, sqlx::Error> {
    let entry = sqlx::query_as!(
        ExerciseLog,
        r#"
            SELECT 
                id,
                log_id,
                exercise_name,
                sets_completed,
                repetitions_completed,
                weight_lifted,
                notes,
                created_at
            FROM exerciselog
            WHERE id = $1
        "#,
        id,
    )
    .fetch_one(&data)
    .await?;

    Ok(entry)
}

pub async fn update_exercise_log_sql(
    data: Pool<Postgres>,
    update: UpdateExerciseLog,
) -> Result<ExerciseLog, sqlx::Error> {
    let result = sqlx::query_as!(
    ExerciseLog,
        r#"
        UPDATE exerciselog
        SET 
            sets_completed = CASE WHEN $1::INTEGER IS NOT NULL THEN $1 ELSE sets_completed END,
            repetitions_completed = CASE WHEN $2::INTEGER IS NOT NULL THEN $2 ELSE repetitions_completed END,
            weight_lifted = CASE WHEN $3::INTEGER IS NOT NULL THEN $3 ELSE weight_lifted END,
            notes = CASE WHEN $4::TEXT IS NOT NULL THEN $4 ELSE notes END
        WHERE log_id = $5 
        RETURNING id, log_id, exercise_name, sets_completed, repetitions_completed, weight_lifted, notes, created_at
        "#,
        update.sets_completed,
        update.repetitions_completed,
        update.weight_lifted,
        update.notes,
        update.log_id,

    )
    .fetch_one(&data)
    .await?;

    Ok(result)
}

pub async fn delete_exercise_log_sql(data: Pool<Postgres>, id: uuid::Uuid) -> u64 {
    let rows_affected = sqlx::query!("DELETE FROM exerciselog WHERE id = $1", id)
        .execute(&data)
        .await
        .unwrap()
        .rows_affected();

    rows_affected
}

pub async fn list_exercise_log_sql(
    data: Pool<Postgres>,
    log_id: uuid::Uuid,
) -> Result<Vec<ExerciseLog>, sqlx::Error> {
    let result = sqlx::query_as!(
        ExerciseLog,
        r#"
        SELECT 
            id,
            log_id,
            exercise_name,
            sets_completed,
            repetitions_completed,
            weight_lifted,
            notes,
            created_at
        FROM exerciselog
        WHERE log_id = $1
        ORDER BY exercise_name ASC
        "#,
        log_id
    )
    .fetch_all(&data)
    .await;
    let plan: Vec<ExerciseLog>;
    match result {
        Ok(entry) => {
            plan = entry;
        }
        Err(_) => {
            plan = vec![];
        }
    }
    Ok(plan)
}
