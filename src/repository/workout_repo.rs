use sqlx::{Pool, Postgres};

use crate::models::workout::{NewWorkout, Workout, UpdateWorkout};

pub async fn create_workout_sql(
    data: Pool<Postgres>,
    new_entry: NewWorkout,
) -> Result<Workout, sqlx::Error> {
    let entry = sqlx::query_as!(
        Workout,
        r#"
            INSERT INTO workout (workout_name, notes)
            VALUES ($1, $2)
            RETURNING id, workout_name, notes, created_at
        "#,
        new_entry.workout_name.to_string(),
        new_entry.notes,
    )
    .fetch_one(&data)
    .await?;

    Ok(entry)
}

pub async fn get_workout_sql(
    data: Pool<Postgres>,
    workout_name: String,
) -> Result<Workout, sqlx::Error> {
    let entry = sqlx::query_as!(
        Workout,
        r#"
            SELECT 
                id, 
                workout_name,
                notes,
                created_at
            FROM workout
            WHERE workout_name = $1
        "#,
        workout_name.to_string(),
    )
    .fetch_one(&data)
    .await
    .unwrap();

    Ok(entry)
}

pub async fn update_workout_sql(
    data: Pool<Postgres>,
    update: UpdateWorkout,
) -> Result<Workout, sqlx::Error> {
    let result = sqlx::query_as!(
    Workout,
        r#"
        UPDATE workout
        SET 
            workout_name = CASE WHEN $1::VARCHAR(255) IS NOT NULL THEN $1 ELSE workout_name END,
            notes = CASE WHEN $2::TEXT IS NOT NULL THEN $2 ELSE notes END
        WHERE id = $3
        RETURNING id, workout_name, notes, created_at
        "#,
        update.workout_name,
        update.notes,
        update.id,

    )
    .fetch_one(&data)
    .await?;

    Ok(result)
}

pub async fn delete_workout_sql(data: Pool<Postgres>, id: uuid::Uuid) -> u64 {
    let rows_affected = sqlx::query!("DELETE FROM workout WHERE id = $1", id)
        .execute(&data)
        .await
        .unwrap()
        .rows_affected();

    rows_affected
}

pub async fn list_workout_sql(
    data: Pool<Postgres>,
) -> Result<Vec<Workout>, sqlx::Error> {
    let result = sqlx::query_as!(
        Workout,
        r#"
        SELECT 
                id, 
                workout_name,
                notes,
                created_at
        FROM workout
        ORDER BY workout_name ASC
        "#,
    )
    .fetch_all(&data)
    .await;
    let plan: Vec<Workout>;
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
