use actix_web::{
    delete, get, post, put,
    web::{scope, Data, Json, Path, ServiceConfig},
    HttpRequest, HttpResponse, Responder,
};
use serde::Deserialize;
use serde_json::json;
use sqlx::SqlitePool;

use crate::services::movie_service::MovieService;

type AResult<T> = actix_web::Result<T>;
pub fn init_movie_route(cfg: &mut ServiceConfig) {
    cfg.service(
        scope("/movies")
            .service(get_movies)
            .service(create_movie)
            .service(get_movie_by_id)
            .service(del_movie_by_id)
            .service(update_movie_by_id),
    );
}

#[get("")]
async fn get_movies(_req: HttpRequest, pool: Data<SqlitePool>) -> AResult<impl Responder> {
    let result = MovieService::get_all_movies(pool.as_ref()).await?;
    let movies = json!(result);

    Ok(HttpResponse::Ok().json(movies))
}

#[get("/{movie_id}")]
async fn get_movie_by_id(pool: Data<SqlitePool>, path: Path<(String,)>) -> AResult<impl Responder> {
    let movie = MovieService::get_movie_by_id(pool.as_ref(), &path.into_inner().0).await?;
    let json_movie = json!(movie);
    Ok(HttpResponse::Ok().json(json_movie))
}

#[post("")]
async fn create_movie(pool: Data<SqlitePool>, movie: Json<NMovie>) -> AResult<impl Responder> {
    let movie = MovieService::create_movie(pool.as_ref(), &movie).await?;
    let json_movie = json!(movie);
    Ok(HttpResponse::Ok().json(json_movie))
}
#[delete("/{movie_id}")]
async fn del_movie_by_id(pool: Data<SqlitePool>, id: Path<String>) -> AResult<impl Responder> {
    MovieService::del_movie_by_id(&pool, &id.into_inner()).await?;

    Ok(HttpResponse::Ok().body("Deleted."))
}

#[derive(Deserialize)]
pub struct NMovie {
    pub title: String,
    pub genre: String,
    pub release_year: String,
}
#[put("/{movie_id}")]
async fn update_movie_by_id(
    pool: Data<SqlitePool>,
    id: Path<String>,
    movie: Json<NMovie>,
) -> AResult<impl Responder> {
    let _res = MovieService::update_by_id(pool.as_ref(), &id, movie.0).await?;
    Ok(HttpResponse::Ok().body("movie update "))
}
