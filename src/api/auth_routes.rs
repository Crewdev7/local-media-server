use std::format;

use actix_http::HttpMessage;
use actix_identity::Identity;
use actix_web::{
    post,
    web::{self, ServiceConfig},
    HttpRequest, HttpResponse, Responder,
};
use serde::Deserialize;
use sqlx::SqlitePool;

use crate::services::user_service::UserService;

type AResult<T> = actix_web::Result<T>;

pub fn init_auth_route(cfg: &mut ServiceConfig) {
    cfg.service(web::resource("/auth").service(signup).service(login));
}

#[post("/signup")]
async fn signup(pool: web::Data<SqlitePool>, n_user: web::Json<NUser>) -> AResult<impl Responder> {
    let res = UserService::create_user(pool.as_ref(), &n_user).await?;

    let msg = format!("user created with id {res} and email {}", n_user.email);

    Ok(HttpResponse::Created().body(msg))
}

#[post("/login")]
async fn login(
    req: HttpRequest,
    pool: web::Data<SqlitePool>,
    n_user: web::Json<NUser>,
) -> AResult<impl Responder> {
    // generates (id,token) and check for email
    let (id, token) = UserService::login_by_email(pool.as_ref(), &n_user).await?;

    // make a custom error to handle it
    Identity::login(&req.extensions(), id.to_string()).unwrap();
    Ok(HttpResponse::Ok().body(token))
}

#[derive(Debug, Deserialize)]
pub struct NUser {
    pub email: String,
    pub pass: String,
}
