use sqlx::{Pool, Postgres};

use crate::models::exercise::{Exercise, NewExercise, UpdateExercise};

pub async fn create_exercise_sql(
    data: Pool<Postgres>,
    new_entry: NewExercise,
) -> Result<Exercise, sqlx::Error> {
    let entry = sqlx::query_as!(
        Exercise,
        r#"
            INSERT INTO exercise (exercise_name, equipment_required, description, instructions, muscle_group_name)
            VALUES ($1, $2, $3, $4, $5)
            RETURNING exercise_name, equipment_required AS "equipment_required: _", description, instructions, muscle_group_name AS "muscle_group_name: _", created_at
        "#,
        new_entry.exercise_name.to_string(),
        new_entry.equipment_required as _,
        new_entry.description.to_string(),
        new_entry.instructions.to_string(),
        new_entry.muscle_group_name as _,
    )
    .fetch_one(&data)
    .await?;

    Ok(entry)
}

pub async fn get_exercise_sql(
    data: Pool<Postgres>,
    name: String,
) -> Result<Exercise, sqlx::Error> {
    let entry = sqlx::query_as!(
        Exercise,
        r#"
            SELECT exercise_name, equipment_required AS "equipment_required: _", description, instructions, muscle_group_name AS "muscle_group_name: _", created_at
            FROM exercise
            WHERE exercise_name = $1
        "#,
        name.to_string(),
    )
    .fetch_one(&data)
    .await?;

    Ok(entry)
}

pub async fn update_exercise_sql(
    data: Pool<Postgres>,
    update: UpdateExercise,
) -> Result<Exercise, sqlx::Error> {
    let result = sqlx::query_as!(
    Exercise,
        r#"
        UPDATE exercise
        SET 
        exercise_name = CASE WHEN $1::VARCHAR(255) IS NOT NULL THEN $1 ELSE exercise_name END,
        equipment_required = CASE WHEN $2::equipmentenum IS NOT NULL THEN $2 ELSE equipment_required END,
        description = CASE WHEN $3::TEXT IS NOT NULL THEN $3 ELSE description END,
        instructions = CASE WHEN $4::TEXT IS NOT NULL THEN $4 ELSE instructions END,
        muscle_group_name = CASE WHEN $5::musclegroupenum IS NOT NULL THEN $5 ELSE muscle_group_name END
        WHERE exercise_name = $1
        RETURNING exercise_name, equipment_required AS "equipment_required: _", description, instructions, muscle_group_name AS "muscle_group_name: _", created_at
        "#,
        update.exercise_name.to_string(),
        update.equipment_required as _,
        update.description,
        update.instructions,
        update.muscle_group_name as _,
    )
    .fetch_one(&data)
    .await?;

    Ok(result)
}

pub async fn delete_exercise_sql(data: Pool<Postgres>, name: String) -> u64 {
    let rows_affected = sqlx::query!("DELETE FROM exercise WHERE exercise_name = $1", name)
        .execute(&data)
        .await
        .unwrap()
        .rows_affected();

    rows_affected
}

pub async fn list_exercises_sql(data: Pool<Postgres>) -> Result<Vec<Exercise>, sqlx::Error> {
    let result = sqlx::query_as!(
        Exercise,
        r#"SELECT 
            exercise_name, 
            equipment_required AS "equipment_required: _", 
            description, 
            instructions, 
            muscle_group_name AS "muscle_group_name: _", 
            created_at
        FROM exercise
        ORDER BY exercise_name ASC
        "#,
    )
    .fetch_all(&data)
    .await;
    let exercises: Vec<Exercise>;
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
