use actix_web::HttpResponse;
use std::env;

use actix_web::ResponseError;
use bcrypt::{hash, DEFAULT_COST};
use chrono::Utc;
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use log::debug;
use serde::{de::DeserializeOwned, Deserialize, Serialize};

use sqlx::SqlitePool;
use thiserror::Error;

use crate::{api::auth_routes::NUser, models::user_model::User};

#[derive(Debug, Error)]
pub enum UserServiceError {
    #[error("User with email {0} already exists")]
    EmailAlreadyExists(String),

    #[error("User with email {0} not found")]
    EmailNotFound(String),

    #[error("Unable to delete user with id {0}")]
    UnableToDelete(String),

    #[error("Invalid password")]
    InvalidPassword(String),

    #[error("Database error: {0}")]
    DbError(#[from] sqlx::Error),

    #[error("Unable to create user with email {0}")]
    UnableToCreate(String),

    #[error("user not found with id {0}")]
    UserNotFound(String),

    #[error("Error while generating JWT token:{0}")]
    TokenError(#[from] jsonwebtoken::errors::Error),

    #[error("Hash/ Decode error: {0}")]
    BCryptError(#[from] bcrypt::BcryptError),

    #[error("{0}")]
    Unkown(String),
}

pub struct UserService;

impl UserService {
    pub async fn get_all_users(pool: &SqlitePool) -> Result<Vec<User>, UserServiceError> {
        Ok(User::get_all(pool).await?)
    }
    pub async fn login_by_email(
        pool: &SqlitePool,
        n_user: &NUser,
    ) -> Result<(i64, String), UserServiceError> {
        let user = Self::get_user_by_email(pool, &n_user.email).await?;

        // Verify hash
        if !user.verify_password(&n_user.pass) {
            debug!("password varification failed for: {:?}", n_user);
            return Err(UserServiceError::InvalidPassword("Invalid password".into()));
        };

        // generate jwt with user details or esnd general error
        let token = generate_jwt_token(&user)?;
        Ok((user.id, token))
    }
    pub async fn create_user(pool: &SqlitePool, n_user: &NUser) -> Result<i32, UserServiceError> {
        // if user is Some(user) return early
        if User::find_by_email(pool, &n_user.email).await?.is_some() {
            return Err(UserServiceError::EmailAlreadyExists(
                n_user.email.to_string(),
            ));
        }

        //  hash & create user if non presest
        let hash_pass = hash(&n_user.pass, DEFAULT_COST)?;
        let user_id = User::create(pool, &n_user.email, &hash_pass).await?;

        return Ok(user_id);
    }

    // can inline if no future scale
    pub async fn get_user_by_id(pool: &SqlitePool, id: &str) -> Result<User, UserServiceError> {
        let user = User::find_by_id(pool, id).await?;

        match user {
            Some(user) => return Ok(user),
            None => return Err(UserServiceError::UserNotFound(id.into())),
        }
    }

    pub async fn get_user_by_email(
        pool: &SqlitePool,
        email: &str,
    ) -> Result<User, UserServiceError> {
        let res = User::find_by_email(pool, email).await?;

        match res {
            Some(user) => return Ok(user),
            None => return Err(UserServiceError::UserNotFound(email.into())),
        }
    }

    pub async fn request_deletion(pool: &SqlitePool, id: &str) -> Result<(), UserServiceError> {
        Ok(User::mark_id_for_deletion(pool, id).await?)
    }

    pub async fn del_user_by_id(pool: &SqlitePool, id: &str) -> Result<(), UserServiceError> {
        if User::delete_by_id(pool, id).await? == 1 {
            Ok(())
        } else {
            Err(UserServiceError::UserNotFound(id.into()))
        }
    }
    //     pub async fn update_password(
    //         pool: &SqlitePool,
    //         user: &User,
    //         pass: &str,
    //     ) -> Result<(), UserServiceError> {
    //         Ok(user
    //             .update_password(pool, pass)
    //             .await
    //             .map_err(|e| UserServiceError::Unkown("unable to update password".into()))?)
    //     }
    // }

    pub async fn update_password_by_id(
        pool: &SqlitePool,
        id: &str,
        pass: &str,
    ) -> Result<(), UserServiceError> {
        Ok(User::update_password_by_id(pool, id, pass)
            .await
            .map_err(|_| UserServiceError::Unkown("unable to update password".into()))?)
    }
}

// fn generate_jwt_tokend<'a>(user: &User) -> Result<String, UserServiceError> {
//     let claims = Claims {
//         id: user.id.to_string(),
//         email: user.email.to_string(),
//         is_admin: user.is_admin.to_string(),
//         plan: user.subscription_plan.to_string(),
//     };
//     let secret_key = env::var("SECRET_KEY").expect("Secret key is not set");
//     let k = EncodingKey::from_secret(secret_key.as_bytes());
//
//     Ok(encode(&Header::default(), &claims, &k)?)
// }
//
impl ResponseError for UserServiceError {
    fn error_response(&self) -> actix_web::HttpResponse<actix_http::body::BoxBody> {
        log::error!("main error obj. e::: {self:?}");
        match self {
            UserServiceError::DbError(_err) => {
                HttpResponse::InternalServerError().body("Internal server1 error")
            }

            UserServiceError::TokenError(_err) => {
                HttpResponse::BadRequest().body("Jwt token error")
            }
            _ => {
                log::error!("Unmapped errors:: {self}");
                HttpResponse::BadRequest().body("Something went wrong")
            }
        }
    }
}
// #[derive(Debug, Serialize)]
// pub struct Claims {
//     id: String,
//     email: String,
//     is_admin: String,
//     plan: String,
// }
#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: String,
    pub email: String,
    pub is_admin: bool,
    pub plan: String,
    pub exp: usize,
}

fn generate_jwt_token(user: &User) -> Result<String, UserServiceError> {
    let exp = Utc::now()
        .checked_add_signed(chrono::Duration::minutes(49))
        .expect("valid timestamp")
        .timestamp();

    let claims = Claims {
        sub: user.id.to_string(),
        email: user.email.to_string(),
        is_admin: user.is_admin == 1,
        plan: user.subscription_plan.to_string(),
        exp: exp as usize,
    };

    let secret_key = env::var("SECRET_KEY").expect("Secret key is not set");
    let key = EncodingKey::from_secret(secret_key.as_bytes());

    Ok(encode(&Header::default(), &claims, &key)?)
}
pub fn decode_jwt_token<T: DeserializeOwned>(token: &str) -> Result<T, UserServiceError> {
    let secret_key = env::var("SECRET_KEY").expect("Secret key is not set");
    let decoding_key = DecodingKey::from_secret(secret_key.as_ref());
    let validation = Validation::default();
    let decode_token = decode::<T>(token, &decoding_key, &validation)?;
    let claims = decode_token.claims;
    Ok(claims)
}
