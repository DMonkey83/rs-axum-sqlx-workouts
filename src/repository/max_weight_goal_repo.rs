use sqlx::{Pool, Postgres};

use crate::models::max_weight_goal::{MaxWeightGoal, NewMaxWeightGoal, UpdateMaxWeightGoal};

pub async fn create_max_weight_goal_sql(
    data: Pool<Postgres>,
    new_entry: NewMaxWeightGoal,
) -> Result<MaxWeightGoal, sqlx::Error> {
    let entry = sqlx::query_as!(
        MaxWeightGoal,
        r#"
            INSERT INTO maxweightgoal (exercise_name, username, goal_weight, notes)
            VALUES ($1, $2, $3, $4 )
            RETURNING id, username, exercise_name, goal_weight, notes, created_at
        "#,
        new_entry.exercise_name.to_string(),
        new_entry.username.to_string(),
        new_entry.goal_weight,
        new_entry.notes
    )
    .fetch_one(&data)
    .await?;

    Ok(entry)
}

pub async fn update_max_weight_goal_sql(
    data: Pool<Postgres>,
    update: UpdateMaxWeightGoal,
) -> Result<MaxWeightGoal, sqlx::Error> {
    let result = sqlx::query_as!(
    MaxWeightGoal,
        r#"
        UPDATE maxweightgoal
        SET 
        goal_weight = CASE WHEN $1::INTEGER IS NOT NULL THEN $1 ELSE goal_weight END,
        notes = CASE WHEN $2::TEXT IS NOT NULL THEN $2 ELSE notes END
        WHERE exercise_name = $3 AND username = $4
        RETURNING id, username, exercise_name, goal_weight, notes, created_at
        "#,
        update.goal_weight,
        update.notes,
        update.exercise_name.to_string(),
        update.username.to_string(),
    )
    .fetch_one(&data)
    .await?;

    Ok(result)
}

pub async fn delete_max_weight_goal_sql(data: Pool<Postgres>, id: uuid::Uuid) -> u64 {
    let rows_affected = sqlx::query!("DELETE FROM maxweightgoal WHERE id = $1", id)
        .execute(&data)
        .await
        .unwrap()
        .rows_affected();

    rows_affected
}

