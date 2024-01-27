use sqlx::{Pool, Postgres};

use crate::models::max_rep_goal::{MaxRepGoal, NewMaxRepGoal, UpdateMaxRepGoal};

pub async fn create_max_rep_goal_sql(
    data: Pool<Postgres>,
    new_entry: NewMaxRepGoal,
) -> Result<MaxRepGoal, sqlx::Error> {
    let entry = sqlx::query_as!(
        MaxRepGoal,
        r#"
            INSERT INTO maxrepgoal (exercise_name, username, goal_reps, notes)
            VALUES ($1, $2, $3, $4 )
            RETURNING id, username, exercise_name, goal_reps, notes, created_at
        "#,
        new_entry.exercise_name.to_string(),
        new_entry.username.to_string(),
        new_entry.goal_reps,
        new_entry.notes
    )
    .fetch_one(&data)
    .await?;

    Ok(entry)
}

pub async fn get_max_rep_goal_sql(
    data: Pool<Postgres>,
    id: uuid::Uuid,
) -> Result<MaxRepGoal, sqlx::Error> {
    let entry = sqlx::query_as!(
        MaxRepGoal,
        r#"
            SELECT id, username, exercise_name, goal_reps, notes, created_at
            FROM maxrepgoal
            WHERE id = $1
        "#,
        id,
    )
    .fetch_one(&data)
    .await?;

    Ok(entry)
}

pub async fn update_max_rep_goal_sql(
    data: Pool<Postgres>,
    update: UpdateMaxRepGoal,
) -> Result<MaxRepGoal, sqlx::Error> {
    let result = sqlx::query_as!(
    MaxRepGoal,
        r#"
        UPDATE maxrepgoal
        SET 
        goal_reps = CASE WHEN $1::INTEGER IS NOT NULL THEN $1 ELSE goal_reps END,
        notes = CASE WHEN $2::TEXT IS NOT NULL THEN $2 ELSE notes END
        WHERE exercise_name = $3 AND username = $4
        RETURNING id, username, exercise_name, goal_reps, notes, created_at
        "#,
        update.goal_reps,
        update.notes,
        update.exercise_name.to_string(),
        update.username.to_string(),
    )
    .fetch_one(&data)
    .await?;

    Ok(result)
}

pub async fn delete_max_rep_goal_sql(data: Pool<Postgres>, id: uuid::Uuid) -> u64 {
    let rows_affected = sqlx::query!("DELETE FROM maxrepgoal WHERE id = $1", id)
        .execute(&data)
        .await
        .unwrap()
        .rows_affected();

    rows_affected
}

