use sqlx::postgres::{PgPool};
use super::resource::{UserSignup};

// create new user
pub async fn create<'a>(mut pool: &'a PgPool, user: UserSignup) -> anyhow::Result<i32>  {
    let row = sqlx::query!(
        r#"
            INSERT INTO users(email, password) 
            VALUES($1, $2)
            RETURNING ID
        "#, user.email, user.password
        )
        .fetch_one(&mut pool)
        .await?;

    Ok(row.id)
}
