use axum::extract::Query;
use sqlx::{Pool, Postgres};

use crate::models::workout_plan::{NewWorkoutPlan, UpdateWorkoutPlan, WorkoutPlan, ListWorkoutPlanResponse};

use super::plan_workout_repo::list_plan_workouts_sql;

pub async fn create_workout_plan_sql(
    data: Pool<Postgres>,
    new_entry: NewWorkoutPlan,
) -> Result<WorkoutPlan, sqlx::Error> {
    let entry = sqlx::query_as!(
        WorkoutPlan,
        r#"
            INSERT INTO workoutplan (username, plan_name, description, start_date, end_date, goal, difficulty, is_public)
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8)
            RETURNING id, username, plan_name, description, start_date, end_date, goal AS "goal: _", difficulty AS "difficulty: _", is_public AS "is_public: _", created_at
        "#,
        new_entry.username.to_string(),
        new_entry.plan_name.to_string(),
        new_entry.description.to_string(),
        new_entry.start_date,
        new_entry.end_date,
        new_entry.goal as _,
        new_entry.difficulty as _,
        new_entry.is_public as _,
    )
    .fetch_one(&data)
    .await?;

    Ok(entry)
}

pub async fn get_workout_plan_sql(
    data: Pool<Postgres>,
    plan_name: String,
    username: String,
) -> Result<WorkoutPlanResponse, sqlx::Error> {
    let entry = sqlx::query_as!(
        WorkoutPlan,
        r#"
            SELECT 
                id, 
                username, 
                plan_name, 
                description, 
                start_date, 
                end_date, 
                goal AS "goal: _", 
                difficulty AS "difficulty: _", 
                is_public AS "is_public: _", 
                created_at
            FROM workoutplan
            WHERE plan_name = $1 AND username = $2
        "#,
        plan_name.to_string(),
        username.to_string(),
    )
    .fetch_one(&data)
    .await?;
    let plan_workouts = list_plan_workouts_sql(data.clone(), entry.id).await.unwrap();

    Ok(entry)
}

pub async fn update_workout_plan_sql(
    data: Pool<Postgres>,
    update: UpdateWorkoutPlan,
) -> Result<WorkoutPlan, sqlx::Error> {
    let result = sqlx::query_as!(
    WorkoutPlan,
        r#"
        UPDATE workoutplan
        SET 
            plan_name = CASE WHEN $1::VARCHAR(255) IS NOT NULL THEN $1 ELSE plan_name END,
            description = CASE WHEN $2::TEXT IS NOT NULL THEN $2 ELSE description END,
            start_date = CASE WHEN $3::timestamp IS NOT NULL THEN $3 ELSE start_date END,
            end_date = CASE WHEN $4::timestamp IS NOT NULL THEN $4 ELSE end_date END,
            goal = CASE WHEN $5::workoutgoalenum IS NOT NULL THEN $5 ELSE goal END,
            difficulty = CASE WHEN $6::difficultyenum IS NOT NULL THEN $6 ELSE difficulty END,
            is_public = CASE WHEN $7::visibilityenum IS NOT NULL THEN $7 ELSE is_public END
        WHERE id = $8
        RETURNING id, username, plan_name, description, start_date, end_date, goal AS "goal: _", difficulty AS "difficulty: _", is_public AS "is_public: _", created_at
        "#,
        update.plan_name,
        update.description,
        update.start_date as _,
        update.end_date as _,
        update.goal as _,
        update.difficulty as _,
        update.is_public as _,
        update.id,

    )
    .fetch_one(&data)
    .await?;

    Ok(result)
}

pub async fn delete_workout_plan_sql(data: Pool<Postgres>, id: uuid::Uuid) -> u64 {
    let rows_affected = sqlx::query!("DELETE FROM workoutplan WHERE id = $1", id)
        .execute(&data)
        .await
        .unwrap()
        .rows_affected();

    rows_affected
}

pub async fn list_workout_plans_sql(
    data: Pool<Postgres>,
    username: Query<String>,
) -> Result<Vec<ListWorkoutPlanResponse>, sqlx::Error> {
    let result = sqlx::query_as!(
        ListWorkoutPlanResponse,
        r#"
        SELECT 
            id, 
            plan_name, 
            description, 
            start_date, 
            end_date, 
            goal AS "goal: _", 
            difficulty AS "difficulty: _", 
            is_public AS "is_public: _", 
            created_at
        FROM workoutplan
        WHERE username = $1
        ORDER BY plan_name ASC
        "#,
        username.to_string(),
    )
    .fetch_all(&data)
    .await;
    let plan: Vec<ListWorkoutPlanResponse>;
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
