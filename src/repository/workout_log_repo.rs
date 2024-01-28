use sqlx::{Pool, Postgres};

use crate::models::workout_log::{NewWorkoutLog, UpdateWorkoutLog, WorkoutLog};

pub async fn create_workout_log_sql(
    data: Pool<Postgres>,
    new_entry: NewWorkoutLog,
) -> Result<WorkoutLog, sqlx::Error> {
    let entry = sqlx::query_as!(
        WorkoutLog,
        r#"
            INSERT INTO workoutlog (
                username, 
                plan_id, 
                workout_id, 
                log_date, 
                rating, 
                fatigue_level, 
                overall_feeling, 
                comments, 
                workout_duration, 
                total_calories_burned, 
                total_distance, 
                total_repetitions, 
                total_sets, 
                total_weight_lifted
        )
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10,
                    $11, $12, $13, $14)
            RETURNING 
                id, 
                username, 
                plan_id, 
                workout_id, 
                log_date, 
                rating AS "rating: _", 
                fatigue_level AS "fatigue_level: _", 
                overall_feeling, 
                comments, 
                workout_duration, 
                total_calories_burned, 
                total_distance, 
                total_repetitions, 
                total_sets, 
                total_weight_lifted,
                created_at
        "#,
        new_entry.username,
        new_entry.plan_id,
        new_entry.workout_id,
        new_entry.log_date,
        new_entry.rating as _,
        new_entry.fatigue_level as _,
        new_entry.overall_feeling,
        new_entry.comments,
        new_entry.workout_duration,
        new_entry.total_calories_burned,
        new_entry.total_distance,
        new_entry.total_repetitions,
        new_entry.total_sets,
        new_entry.total_weight_lifted,
    )
    .fetch_one(&data)
    .await?;

    Ok(entry)
}

pub async fn get_workout_log_sql(
    data: Pool<Postgres>,
    id: uuid::Uuid,
) -> Result<WorkoutLog, sqlx::Error> {
    let entry = sqlx::query_as!(
        WorkoutLog,
        r#"
            SELECT 
                id,
                username,
                plan_id,
                workout_id,
                log_date,
                rating AS "rating: _",
                fatigue_level AS "fatigue_level: _",
                overall_feeling,
                comments,
                workout_duration,
                total_calories_burned,
                total_distance,
                total_repetitions,
                total_sets,
                total_weight_lifted,
                created_at
            FROM workoutlog
            WHERE id = $1
        "#,
        id,
    )
    .fetch_one(&data)
    .await?;

    Ok(entry)
}

pub async fn update_workout_log_sql(
    data: Pool<Postgres>,
    update: UpdateWorkoutLog,
) -> Result<WorkoutLog, sqlx::Error> {
    let result = sqlx::query_as!(
    WorkoutLog,
        r#"
        UPDATE workoutlog
        SET 
            log_date = CASE WHEN $1::timestamp IS NOT NULL THEN $1 ELSE log_date END,
            rating = CASE WHEN $2::ratingenum IS NOT NULL THEN $2 ELSE rating END,
            fatigue_level = CASE WHEN $3::fatiguelevelenum IS NOT NULL THEN $3 ELSE fatigue_level END,
            overall_feeling = CASE WHEN $4::VARCHAR(255) IS NOT NULL THEN $4 ELSE overall_feeling END,
            comments = CASE WHEN $5::VARCHAR(255) IS NOT NULL THEN $5 ELSE comments END,
            workout_duration = CASE WHEN $6::VARCHAR(255) IS NOT NULL THEN $6 ELSE workout_duration END,
            total_calories_burned = CASE WHEN $7::INTEGER IS NOT NULL THEN $7 ELSE total_calories_burned END,
            total_distance = CASE WHEN $8::INTEGER IS NOT NULL THEN $8 ELSE total_distance END,
            total_repetitions = CASE WHEN $9::INTEGER IS NOT NULL THEN $9 ELSE total_repetitions END,
            total_sets = CASE WHEN $10::INTEGER IS NOT NULL THEN $10 ELSE total_sets END,
            total_weight_lifted = CASE WHEN $11::INTEGER IS NOT NULL THEN $11 ELSE total_weight_lifted END
            WHERE username = $12 AND plan_id = $13 AND workout_id = $14
            RETURNING 
                id, 
                username, 
                plan_id, 
                workout_id, 
                log_date, 
                rating AS "rating: _", 
                fatigue_level AS "fatigue_level: _", 
                overall_feeling, 
                comments, 
                workout_duration, 
                total_calories_burned, 
                total_distance, 
                total_repetitions, 
                total_sets, 
                total_weight_lifted,
                created_at
        "#,
        update.log_date as _,
        update.rating as _,
        update.fatigue_level as _,
        update.overall_feeling,
        update.comments,
        update.workout_duration,
        update.total_calories_burned,
        update.total_distance,
        update.total_repetitions,
        update.total_sets,
        update.total_weight_lifted,
        update.username,
        update.plan_id,
        update.workout_id,
    )
    .fetch_one(&data)
    .await?;

    Ok(result)
}

pub async fn delete_workout_log_sql(data: Pool<Postgres>, id: uuid::Uuid) -> u64 {
    let rows_affected = sqlx::query!("DELETE FROM workoutlog WHERE id = $1", id)
        .execute(&data)
        .await
        .unwrap()
        .rows_affected();

    rows_affected
}

pub async fn list_workout_log_sql(
    data: Pool<Postgres>,
    username: String,
    plan_id: uuid::Uuid,
) -> Result<Vec<WorkoutLog>, sqlx::Error> {
    let result = sqlx::query_as!(
        WorkoutLog,
        r#"
        SELECT 
            id,
            username,
            plan_id,
            workout_id,
            log_date,
            rating AS "rating: _",
            fatigue_level AS "fatigue_level: _",
            overall_feeling,
            comments,
            workout_duration,
            total_calories_burned,
            total_distance,
            total_repetitions,
            total_sets,
            total_weight_lifted,
            created_at
        FROM workoutlog
        WHERE username = $1 AND plan_id = $2
        ORDER BY log_date ASC
        "#,
        username,
        plan_id,
    )
    .fetch_all(&data)
    .await;
    let exercises: Vec<WorkoutLog>;
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
