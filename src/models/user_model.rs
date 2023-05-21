use bcrypt::verify;
use serde::{Deserialize, Serialize};
use sqlx::{query, query_as, FromRow, SqlitePool};

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct User {
    pub id: i64,
    pub email: String,
    #[serde(skip_serializing)]
    pub password: String,
    pub subscription_plan: String,
    pub is_banned: i64,
    created_at: String,
    updated_at: String,
    pub is_admin: i64,
    pub data_usage: i64,
}
type SError<T> = sqlx::Result<T>;

impl User {
    pub async fn create(pool: &SqlitePool, email: &str, pass: &str) -> SError<i32> {
        let insert_result = sqlx::query!(
            "INSERT INTO users ( email,password) VALUES (?, ?)",
            email,
            pass
        )
        .execute(pool)
        .await?;

        Ok(insert_result.last_insert_rowid() as i32)
    }
    pub async fn find_by_email(pool: &SqlitePool, email: &str) -> SError<Option<User>> {
        let user = query_as!(User, r#"SELECT * FROM users WHERE  email = ?"#, email)
            .fetch_optional(pool)
            .await?;
        Ok(user)
    }
    pub async fn find_by_id(pool: &SqlitePool, id: &str) -> SError<Option<User>> {
        let user = query_as!(User, r#"SELECT * FROM users WHERE  id= ?"#, id)
            .fetch_optional(pool)
            .await?;
        Ok(user)
    }
    // Todo  without password or map it  to remove  it or  create new struct for this one;
    pub async fn get_all(pool: &SqlitePool) -> SError<Vec<User>> {
        Ok(query_as!(User, "SELECT *  FROM  users")
            .fetch_all(pool)
            .await?)
    }

    pub async fn delete_by_id(pool: &SqlitePool, id: &str) -> SError<u64> {
        let res = query!("DELETE  FROM  users WHERE id = ?", id)
            .execute(pool)
            .await?;
        Ok(res.rows_affected())
    }

    pub async fn update_password(&self, pool: &SqlitePool, new_password: &str) -> SError<()> {
        sqlx::query!(
            r#"UPDATE users SET password = ? WHERE id = ?"#,
            new_password,
            self.id
        )
        .execute(pool)
        .await?;
        Ok(())
    }
    pub async fn update_password_by_id(
        pool: &SqlitePool,
        id: &str,
        new_password: &str,
    ) -> SError<()> {
        sqlx::query!(
            r#"UPDATE users SET password = ? WHERE id = ?"#,
            new_password,
            id
        )
        .execute(pool)
        .await?;
        Ok(())
    }
    pub async fn mark_id_for_deletion(pool: &SqlitePool, id: &str) -> SError<()> {
        let res = sqlx::query!(
            "INSERT INTO del_req (user_id) VALUES (?) ON CONFLICT  DO NOTHING",
            id
        )
        .execute(pool)
        .await?;

        Ok(())
    }
    pub fn verify_password(&self, password: &str) -> bool {
        verify(password, &self.password).unwrap_or_else(|err| {
            log::error!("password verification: {err}");
            false
        })
        // match verify(password, &self.password) {
        //     Ok(result) => result,
        //     Err(err) => {
        //         log::error!("password verify: {err}");
        //         false
        //     }
    }
}
