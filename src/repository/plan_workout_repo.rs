use sqlx::{Pool, Postgres};

use crate::models::plan_workout::{PlanWorkout, NewPlanWorkout, UpdatePlanWorkout, PlanWorkoutResponse};

pub async fn create_plan_workout_sql(
    data: Pool<Postgres>,
    new_entry: NewPlanWorkout,
) -> Result<PlanWorkout, sqlx::Error> {
    let entry = sqlx::query_as!(
        PlanWorkout,
        r#"
            INSERT INTO planworkout (plan_id, workout_id, workout_day, notes)
            VALUES ($1, $2, $3, $4)
            RETURNING id, plan_id, workout_id, workout_day AS "workout_day: _", notes, created_at
        "#,
        new_entry.plan_id,
        new_entry.workout_id,
        new_entry.workout_day as _,
        new_entry.notes
    )
    .fetch_one(&data)
    .await?;

    Ok(entry)
}

pub async fn get_plan_workout_sql(
    data: Pool<Postgres>,
    id: uuid::Uuid,
) -> Result<PlanWorkoutResponse, sqlx::Error> {
    let entry = sqlx::query_as!(
        PlanWorkoutResponse,
        r#"
            SELECT 
                planworkout.id as "id: _",
                planworkout.plan_id AS "plan_id: _",
                planworkout.workout_id AS "workout_id: _", 
                planworkout.workout_day AS "workout_day: _",
                planworkout.notes AS "notes: _",
                workout.workout_name AS "workout_name: _",
                workout.notes AS "workout_notes: _",
                planworkout.created_at AS "created_at: _"
            FROM planworkout
            LEFT JOIN workout ON workout.id = planworkout.workout_id
            WHERE planworkout.id = $1
        "#,
        id,
    )
    .fetch_one(&data)
    .await?;

    Ok(entry)
}

pub async fn update_plan_workout_sql(
    data: Pool<Postgres>,
    update: UpdatePlanWorkout,
) -> Result<PlanWorkout, sqlx::Error> {
    let result = sqlx::query_as!(
    PlanWorkout,
        r#"
        UPDATE planworkout
        SET 
            workout_day = CASE WHEN $1::workoutdayenum IS NOT NULL THEN $1 ELSE workout_day END,
            notes = CASE WHEN $2::TEXT IS NOT NULL THEN $2 ELSE notes END
        WHERE id = $3
        RETURNING id, plan_id, workout_id, workout_day AS "workout_day: _", notes, created_at
        "#,
        update.workout_day as _,
        update.notes,
        update.id,
    )
    .fetch_one(&data)
    .await?;

    Ok(result)
}

pub async fn delete_plan_workout_sql(data: Pool<Postgres>, id: uuid::Uuid) -> u64 {
    let rows_affected = sqlx::query!("DELETE FROM planworkout WHERE id = $1", id)
        .execute(&data)
        .await
        .unwrap()
        .rows_affected();

    rows_affected
}

pub async fn list_plan_workouts_sql(data: Pool<Postgres>, plan_id: uuid::Uuid) -> Result<Vec<PlanWorkoutResponse>, sqlx::Error> {
    let result = sqlx::query_as!(
        PlanWorkoutResponse,
        r#"
        SELECT 
            planworkout.id as "id: _",
            planworkout.plan_id AS "plan_id: _",
            planworkout.workout_id AS "workout_id: _", 
            planworkout.workout_day AS "workout_day: _",
            planworkout.notes AS "notes: _",
            workout.workout_name AS "workout_name: _",
            workout.notes AS "workout_notes: _",
            planworkout.created_at AS "created_at: _"
        FROM planworkout
        LEFT JOIN workout ON workout.id = planworkout.workout_id
        WHERE planworkout.plan_id = $1
        ORDER BY workout.workout_name ASC
        "#,
        plan_id
    )
    .fetch_all(&data)
    .await;
    let exercises: Vec<PlanWorkoutResponse>;
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
