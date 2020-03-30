use sqlx::postgres::{PgPool};
use super::resource::{UserSignup, hash_password};

// create new user
pub async fn create<'a>(mut pool: &'a PgPool, user: UserSignup) -> anyhow::Result<i32>  {
    let hashed_password = hash_password(user.password.clone())?;

    let row = sqlx::query!(
        r#"
            INSERT INTO users(email, password) 
            VALUES($1, $2)
            RETURNING ID
        "#, user.email, hashed_password
        )
        .fetch_one(&mut pool)
        .await?;

    Ok(row.id)
}
