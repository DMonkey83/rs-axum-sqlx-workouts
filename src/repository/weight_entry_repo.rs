use sqlx::{Pool, Postgres};

use crate::models::weight_entry::{NewWeightEntry, WeightEntry, UpdateWeightEntry};



pub async fn create_weight_entry_sql(
    data: Pool<Postgres>,
    new_entry: NewWeightEntry,
) -> Result<WeightEntry, sqlx::Error> {
    let entry = sqlx::query_as!(
        WeightEntry,
        r#"
            INSERT INTO weightentry (username, entry_date, weight, notes)
            VALUES ($1, $2, $3, $4)
            RETURNING id, username, entry_date, weight, notes, created_at
        "#,
        new_entry.username.to_string(),
        new_entry.entry_date,
        new_entry.weight,
        new_entry.notes.to_string(),
    )
    .fetch_one(&data)
    .await?;

    Ok(entry)
}

pub async fn update_weight_entry_sql(
    data: Pool<Postgres>,
    update: UpdateWeightEntry,
) -> Result<WeightEntry, sqlx::Error> {
    let result = sqlx::query_as!(
    WeightEntry,
        r#"
        UPDATE weightentry
        SET 
        entry_date = CASE WHEN $1::TIMESTAMP IS NOT NULL THEN $1 ELSE entry_date END,
        weight = CASE WHEN $2::INTEGER IS NOT NULL THEN $2 ELSE weight END,
        notes = CASE WHEN $3::VARCHAR(255) IS NOT NULL THEN $3 ELSE notes END
        WHERE id = $4 AND username = $5
        RETURNING id, username, entry_date, weight, notes, created_at
        "#,
        update.entry_date as _,
        update.weight,
        update.notes,
        update.id,
        update.username
    )
    .fetch_one(&data)
    .await?;

    Ok(result)
}

pub async fn delete_weight_entry_sql(
    data: Pool<Postgres>,
    username: String,
) -> u64 {
    let rows_affected = sqlx::query!("DELETE FROM weightentry WHERE username = $1", username)
        .execute(&data)
        .await
        .unwrap()
        .rows_affected();

    rows_affected
}

pub async fn list_weight_entry_sql(
    data: Pool<Postgres>,
    username: String,
) -> Vec<WeightEntry> {
    let result = sqlx::query_as!(
        WeightEntry,
        r#"SELECT *
            FROM weightentry 
            WHERE username = $1 
            ORDER BY entry_date DESC
        "#,
        username
    )
    .fetch_all(&data)
    .await;
    let weight_entry: Vec<WeightEntry>;
    match result {
        Ok(entry) => {
            weight_entry = entry;
        }
        Err(_) => {
            weight_entry = vec![];
        }
    }
    weight_entry
}
