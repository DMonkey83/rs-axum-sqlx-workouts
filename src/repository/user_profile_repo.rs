
use sqlx::{Pool, Postgres};

use crate::models::user_profile::{NewUserProfile, UserProfile, UpdateUserProfile, UserProfileResponse, user_profile_response};

use super::weight_entry_repo::list_weight_entry_sql;



pub async fn create_profile_sql(
    data: Pool<Postgres>,
    new_user_profile: NewUserProfile,
) -> Result<UserProfile, sqlx::Error> {
    let entry =sqlx::query_as!(
        UserProfile,
                    r#"INSERT INTO userprofile (username, first_name, last_name,email, age, gender, height, preferred_weight_unit, preferred_height_unit)
                    VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9)
                    RETURNING id, username, first_name, last_name, email, age, gender AS "gender: _", height, preferred_weight_unit AS "preferred_weight_unit: _", preferred_height_unit AS "preferred_height_unit!: _", created_at"#,
        new_user_profile.username.to_string(),
        new_user_profile.first_name.to_string(),
        new_user_profile.last_name.to_string(),
        new_user_profile.email.to_string(),
        new_user_profile.age,
        new_user_profile.gender as _,
        new_user_profile.height,
        new_user_profile.preferred_weight_unit as _,
        new_user_profile.preferred_height_unit as _,

    )
    .fetch_one(&data)
    .await?;

    Ok(entry)
}

pub async fn update_profile_sql(
    data: Pool<Postgres>,
    update: UpdateUserProfile,
) -> Result<UserProfile, sqlx::Error> {
    let result = sqlx::query_as!(
        UserProfile,
        r#"UPDATE userprofile 
            SET 
                first_name = CASE WHEN $1::VARCHAR(255) IS NOT NULL THEN $1 ELSE first_name END,
                last_name = CASE WHEN $2::VARCHAR(255) IS NOT NULL THEN $2 ELSE last_name END,
                age = CASE WHEN $3::INT IS NOT NULL THEN $3 ELSE age END,
                gender = CASE WHEN $4::genderenum IS NOT NULL THEN $4 ELSE gender END,
                height = CASE WHEN $5::INT IS NOT NULL THEN $5 ELSE height END,
                preferred_weight_unit = CASE WHEN $6::weightenum IS NOT NULL THEN $6 ELSE preferred_weight_unit END,
                preferred_height_unit = CASE WHEN $7::heightenum IS NOT NULL THEN $7 ELSE preferred_height_unit END,
                email = CASE WHEN $8::VARCHAR(255) IS NOT NULL THEN $8 ELSE email END
            WHERE username = $9
            RETURNING username, id, first_name, email, last_name, age, gender AS "gender: _", height, preferred_weight_unit AS "preferred_weight_unit: _", preferred_height_unit AS "preferred_height_unit: _", created_at
        "#,
        update.first_name,
        update.last_name,
        update.age,
        update.gender as _,
        update.height,
        update.preferred_weight_unit as _,
        update.preferred_height_unit as _,
        update.email,
        update.username,

    )
    .fetch_one(&data)
    .await?;

    Ok(result)
}

pub async fn delete_profile_sql(
    data: Pool<Postgres>,
    username: String,
) -> u64 {
    let rows_affected = sqlx::query!("DELETE FROM userprofile  WHERE username = $1", username)
        .execute(&data)
        .await
        .unwrap()
        .rows_affected();

    rows_affected
}

pub async fn get_user_profile_sql(
    data: Pool<Postgres>,
    username: String,
) -> Result<UserProfileResponse, sqlx::Error> {
    let profile = sqlx::query_as!(
        UserProfile,
        r#"SELECT 
                id,
                username, 
                first_name, 
                email, 
                last_name, 
                age, 
                gender AS "gender: _", 
                height, preferred_weight_unit AS "preferred_weight_unit: _", 
                preferred_height_unit AS "preferred_height_unit: _", 
                created_at 
            FROM userprofile 
            WHERE username = $1
        "#,
        username
    )
    .fetch_one(&data)
    .await?;
    let weight_entries = list_weight_entry_sql(data, username).await;
    Ok(user_profile_response(profile, weight_entries))
}
