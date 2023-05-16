// use std::env;
//
// use bcrypt::{hash, verify, DEFAULT_COST};
// use jsonwebtoken::{encode, Header};
// use sqlx::{sqlite, FromRow, SqlitePool};
// const DEFAULT_SECRET_KEY: &str = "secret";
// const JWT_SECRET: String = env::var("JWT_SECRET").unwrap_or_else(|| DEFAULT_SECRET_KEY);
// #[derive(Debug, FromRow)]
// pub struct User {
//     id: String,
//     name: String,
//     email: String,
//     password: String,
//     subscription_plan: String,
//     is_banned: bool,
//     created_at: String,
//     is_admin: bool,
//     data_usage: i64,
// }
//
// impl User {
//     pub async fn create(email: &str, password: &str, pool: &SqlitePool) -> sqlx::Result<User> {
//         //     let exist_user = sqlx::query(
//         //         r#"SELECT * FROM users
//         // WHERE email = ?"#,
//         //     )
//         //     .bind(email)
//         //     .fetch_optional(pool)
//         //     .await?
//         //     .is_some();
//         //
//         //     if exist_user {
//         //         log::info!("User already exists");
//         //         return Err(sqlx::Error::RowNotFound);
//         //     }
//         //     // hash password and insert user record into database
//         //     let hashed_password = hash(password, DEFAULT_COST).unwrap();
//         //
//         //     let user_id = sqlx::query(
//         //         "INSERT INTO users (email, password,subscription_plan,
//         //                             data_usage)
//         //             VALUES (?,?,'basic','0')",
//         //     )
//         //     .bind(email.to_lowercase())
//         //     .bind(password)
//         //     .fetch_one(pool)
//         //     .await?;
//         unimplemented!()
//     }
//     pub async fn login(
//         pool: &SqlitePool,
//         email: &str,
//         password: &str,
//     ) -> Result<User, sqlx::Error> {
//         let user = sqlx::query_as::<_, User>("SELECT * FROM users WHERE email = ?")
//             .bind(email)
//             .fetch_optional(pool)
//             .await?;
//         match user {
//             Some(user) => {
//                 if verify(password, &user.password).is_ok() {
//                     return Ok(user);
//                 } else {
//                     Err(sqlx::Error::RowNotFound)
//                 }
//             }
//             _ => Err(sqlx::Error::PoolClosed),
//         }
//     }
// }
//
//
//
//

use std::{fs, io};

use serde::{Deserialize, Serialize};
use sqlx::{Error, FromRow, SqlitePool};

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct User {
    id: i32,
    name: String,
    email: String,
    password: String,
    subscription_plan: String,
    is_banned: bool,
    created_at: String,
    is_admin: bool,
    data_usage: i64,
}
use thiserror::Error;
#[derive(Debug, Error)]
pub enum MyError {
    #[error("Database error: {}",.0)]
    DbError(#[from] sqlx::Error),
    #[error("inpuuut  /o  error: {}",.0)]
    IooError(#[from] io::Error),
    #[error("inpuuut,,,,22  /o  error: {}",.0)]
    IooError2(#[source] Box<dyn std::error::Error+Send+Sync>),
    #[error("User Not Found  with   {}", user)]
    UserNotFound { user: String },
    #[error("unkwon erro")]
    Unknown,
}
impl User {
    #[track_caller]
    pub async fn find_user_by_email(pool: &SqlitePool, email: &str) -> Result<User, MyError> {

        let ff = fs::File::open("/e")?;
        let user = sqlx::query_as::<_, User>("SELECT * FROM users WHERE   ueoh = ?")
            .bind(email)
            .fetch_optional(pool)
            .await
            .map_err(|err| match err {
                sqlx::Error::RowNotFound => {
                    let e = io::Error::new(
                        io::ErrorKind::NotFound,
                        format!("mapped to the error : {:?}", err),
                    );
                    return MyError::IooError(e);
                }
                sqlx::Error::Database(e) => {
                    let e = io::Error::new(
                        io::ErrorKind::NotFound,
                        format!("mapped to the error : {:?}", e),
                    );
                    return MyError::IooError(e);
                }
                _ => return MyError::DbError(err),
            })?;
        let ff = fs::File::open("/e")?;
        match user {
            Some(user) => Ok(user),
            None => Err(MyError::UserNotFound { user: email.into() }),
        }
    }
    pub async fn update_password(pool: &SqlitePool, id: i32, password: &str) -> Result<(), Error> {
        sqlx::query(r#"UPDATE users SET password = ? WHERE id = ?"#)
            .bind(password)
            .bind(id)
            .execute(pool)
            .await?;

        Ok(())
    }
}
