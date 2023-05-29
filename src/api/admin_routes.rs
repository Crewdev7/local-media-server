use actix_identity::Identity;
use actix_web::{
    error::ErrorBadRequest,
    get, post, web,
    web::{resource, scope, Data, ServiceConfig},
    HttpRequest, HttpResponse, Responder,
};
use serde::{Deserialize, Serialize};
use serde_json::json;
use sqlx::SqlitePool;

use crate::services::{movie_service::MovieService, user_service::UserService};

type AResult<T> = actix_web::Result<T>;
pub fn init_admin_route(cfg: &mut ServiceConfig) {
    cfg.service(
        scope("/admin")
            .service(scope("/media").service(upload_movie))
            // view all users
            .service(
                scope("/users")
                    .service(get_all_users)
                    // manage users individually
                    .service(resource("/by_id")),
            )
            .service(scope("/by_id")),
    );
}


use actix_web::{Error};
use futures::{StreamExt, TryStreamExt};
use std::io::Write;


// #[post("")]
// async fn upload_movie(mut payload: web::Payload) -> Result<HttpResponse, Error> {
// let mut bytes = BytesMut::new();
//
// let mut file = web::block(|| MovieService::upload_file()).await?;
//
// while let Some(item) = payload.next().await {
//     let data = item?;
//
//     log::info!("data: {data:?}");
//     bytes.extend_from_slice(&data);
// }
//
// match file {
//     Ok(ref mut file) => {
//         file.write_all(&bytes)?;
//
//         log::info!("file written wit data");
//     }
//     Err(e) => {
//         log::error!("err: {e}");
//         return Err(ErrorBadRequest("OUE"));
//     }
// };

#[derive(Serialize, Deserialize)]
struct MyObj {
    name: String,
    number: i32,
}

const MAX_SIZE: usize = 262_14493939; // max payload size is 256k

#[post("")]
async fn upload_movie(mut payload: web::Payload) -> Result<HttpResponse, Error> {
    // payload is a stream of Bytes objects
    let mut body = web::BytesMut::new();
    while let Some(chunk) = payload.next().await {
        let chunk = chunk?;
        // limit max size of in-memory payload
        if (body.len() + chunk.len()) > MAX_SIZE {
            return Err(actix_web::error::ErrorBadRequest("overflow"));
        }
        body.extend_from_slice(&chunk);
    }

    let _file = MovieService::upload_file()
        .unwrap()
        .write_all(&body)
        .unwrap();

    // body is loaded, now we can deserialize serde-json
    Ok(HttpResponse::Ok().body("ok")) // <- send response
}
#[get("")]
async fn get_all_users(req: HttpRequest, pool: Data<SqlitePool>) -> AResult<impl Responder> {
    // Retrieve the authorization header from the request
    let auth_header = req.headers().get("Authorization");

    let token = auth_header
        .and_then(|header_value| header_value.to_str().ok())
        .filter(|header_value| header_value.starts_with("Bearer "))
        .and_then(|header_value| Some(header_value.trim_start_matches("Bearer ").trim().to_owned()))
        .and_then(|token| if !token.is_empty() { Some(token) } else { None })
        .ok_or_else(|| ErrorBadRequest("no oauth provided"))?;

    if token.is_empty() {
        return Err(ErrorBadRequest("token is empty"));
    };
    let users = UserService::get_all_users(pool.as_ref()).await?;
    let json_user = json!(users);

    Ok(HttpResponse::Ok().json(json_user))
}

#[get("")]
async fn get_all_liked_movie(
    identity: Identity,
    pool: Data<SqlitePool>,
) -> AResult<impl Responder> {
    let user_id = match identity.id().ok() {
        Some(id) => id,
        None => return Ok(HttpResponse::BadRequest().body("You must be logged in")),
    };

    let result = MovieService::get_all_liked_movie(pool.as_ref(), &user_id).await?;
    let movies = json!(result);

    Ok(HttpResponse::Ok().json(movies))
}
