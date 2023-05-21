use actix_identity::Identity;
use actix_web::{
    delete,
    error::{ErrorBadRequest, ErrorForbidden},
    get, put,
    web::{scope, Data, Json, Path, ServiceConfig},
    HttpRequest, HttpResponse, Responder,
};
use log::info;
use serde::Deserialize;
use serde_json::json;
use sqlx::SqlitePool;

use crate::services::user_service::{decode_jwt_token, Claims, UserService};
type AResult<T> = actix_web::Result<T>;
pub fn init_user_route(cfg: &mut ServiceConfig) {
    cfg.service(
        scope("/user")
            .service(get_user)
            .service(get_user_by_id)
            .service(del_user_by_id)
            .service(update_password),
    );
}

#[get("")]
async fn get_user(req: HttpRequest, _pool: Data<SqlitePool>) -> AResult<impl Responder> {
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

    let claim = decode_jwt_token::<Claims>(token.as_ref())?;
    let user = json!(claim);

    Ok(HttpResponse::Ok().json(user))
}

#[get("/{user_id}")]
async fn get_user_by_id(pool: Data<SqlitePool>, path: Path<(String,)>) -> AResult<impl Responder> {
    let user = UserService::get_user_by_id(pool.as_ref(), &path.into_inner().0).await?;
    let json_user = json!(user);
    Ok(HttpResponse::Ok().json(json_user))
}

#[delete("")]
async fn del_user_by_id(
    pool: Data<SqlitePool>,
    identity: Option<Identity>,
) -> AResult<impl Responder> {
    // let id = identity
    //     .and_then(|id| id.id().ok())
    //     .ok_or(ErrorForbidden("login to delete your profile"))?;

    let id = identity.and_then(|id| Some(id));
    match id {
        Some(id) => {
            let user_id = id
                .id()
                .ok()
                .ok_or(ErrorForbidden("Login to delete profile"))?;

            UserService::request_deletion(&pool, &user_id).await?;
            id.logout()
        }
        None => {
            return Err(ErrorBadRequest(
                "Please login again. Your sesison has been deleted.",
            ))
        }
    };

    Ok(HttpResponse::Ok().body("success login again in 1hour to cancel."))
}

//TODO attac user object to session for directly call its methods on user.
// temp using id and update password by calling construcstor
#[derive(Deserialize)]
struct UpdatePass {
    pass: String,
}
#[put("/{user_id}/update_pass")]
async fn update_password(
    pool: Data<SqlitePool>,
    identity: Identity,
    user_id: Path<String>,
    pass: Json<UpdatePass>,
) -> AResult<impl Responder> {
    let id = identity.id().expect("error while extracting id");
    if user_id.clone() == id.clone() {
        let _res = UserService::update_password_by_id(pool.as_ref(), &id, &pass.pass).await?;
        let msg = format!("user deleted with id {}", &user_id);
        info!("{msg}");
        Ok(HttpResponse::Ok().body(msg))
    } else {
        Err(ErrorForbidden("please login first"))
    }
}
