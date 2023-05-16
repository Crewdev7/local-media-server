use std::env;

use actix_http::StatusCode;
use actix_web::{
    error::ErrorUnauthorized, post, web, HttpMessage, HttpRequest, HttpResponse, Responder,
    Result as AResult,
};
use serde::{Deserialize, Serialize};
use sqlx::{self, SqlitePool};

use bcrypt::{hash, verify, DEFAULT_COST};

use crate::user::{self, model::User};

#[derive(Debug, Deserialize)]
struct Form {
    email: String,
    password: String,
}
#[derive(Debug, Serialize, Deserialize)]
struct Claims {
    sub: i64,
}
#[post("/signup")]
#[track_caller]
pub async fn signup(pool: web::Data<SqlitePool>, form: web::Json<Form>) -> AResult<impl Responder> {
    let email = &form.email;
    let pass = &form.password;
    let user = User::find_user_by_email(pool.get_ref(), "ok@ok.com")
        .await
        .map_err(|err| {
            println!("we got error display  {}", err);
            println!("we got Debug error {:?}", err);
            return err;
        })
        .unwrap();println!("we go user {user:?}");
    // match user {
    //     Ok(user) => println!("we go user {user:?}"),
    //     Err(err) => println!("we got err {err}"),
    // }
    Ok("")
}

#[post("/login")]
pub async fn login(pool: web::Data<SqlitePool>, form: web::Json<Form>) -> AResult<HttpResponse> {
    unimplemented!()
}

// async fn auth_middleware(req: HttpRequest, pool: web::Data<SqlitePool>) -> AResult<HttpResponse> {
//     let auth_header = req.headers().get("Authorization");
//     if let Some(auth_header) = auth_header {
//         let auth_header_str = auth_header.to_str().unwrap();
//
//         if auth_header_str.starts_with("Bearer") {
//             let token = auth_header_str[7..]..to_owned();
//
//             //decode jwt token
//             let decoded_token = decode(
//                 &token,
//                 env::var("JWT_SECRET").unwrap(),
//                 &Validation::new(jsonwebtoken::Algorithm::HS256),
//             );
//
//             if let Ok(decoded_token) = decoded_token {
//                 let user = sqlx::query("SELECT * FROM users WHERE id =?")
//                     .bind(decoded_token.claims.sub)
//                     .fetch_optional(&pool.as_ref())
//                     .await
//                     .unwrap();
//             };
//             if let Some(user) = user {
//                 let mut new_req = req.clone();
//                 new_req.extensions_mut().insert(user);
//                 return Ok(new_rq);
//             }
//         }
//     }
//     Err(ErrorUnauthorized("Unauthorized"))
// }
