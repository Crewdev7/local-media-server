use actix_identity::Identity;
use actix_web::HttpResponse;
use actix_web::ResponseError;

use sqlx::SqlitePool;
use thiserror::Error;

use crate::api::movie_routes::NMovie;
use crate::models::movie_model::Movie;

#[derive(Debug, Error)]
pub enum MovieSError {
    #[error("Movie {0} already exists")]
    AlreadyExists(String),

    #[error("{0} not found")]
    NotFound(String),

    #[error("Unable to delete {0}")]
    UnableToDelete(String),

    #[error("Database error: {0}")]
    DbError(#[from] sqlx::Error),

    #[error("Unable to create {0}")]
    UnableToCreate(String),

    #[error("{0}")]
    Unkown(String),
}

pub struct MovieService;

impl MovieService {
    pub async fn get_all_movies(pool: &SqlitePool) -> Result<Vec<Movie>, MovieSError> {
        Ok(Movie::get_all(pool).await?)
    }


    pub async fn get_all_liked_movie(pool: &SqlitePool,user_id: &str) -> Result<Vec<Movie>, MovieSError> {

        Ok(Movie::get_all_liked_movie(pool,user_id).await?)
    }

    pub async fn create_movie(pool: &SqlitePool, n_movie: &NMovie) -> Result<i32, MovieSError> {
        let movie_id =
            Movie::create(pool, &n_movie.title, &n_movie.genre, &n_movie.release_year).await?;

        return Ok(movie_id);
    }

    // can inline if no future scale
    pub async fn get_movie_by_id(pool: &SqlitePool, id: &str) -> Result<Movie, MovieSError> {
        let movie = Movie::find_by_id(pool, id).await?;

        match movie {
            Some(movie) => return Ok(movie),
            None => return Err(MovieSError::NotFound(id.into())),
        }
    }

    // TODO
    // make it return Option<Vec<moive>
    // pub async fn get_movie_by_title(pool: &SqlitePool, title: &str) -> Result<Movie, MovieSError> {
    //     let res = Movie::find_by_title(pool, title).await?;
    //
    //     match res {
    //         Some(movie) => return Ok(movie),
    //         None => return Err(MovieSError::NotFound(title.into())),
    //     }
    // }

    pub async fn del_movie_by_id(pool: &SqlitePool, id: &str) -> Result<(), MovieSError> {
        if Movie::delete_by_id(pool, id).await? == 1 {
            Ok(())
        } else {
            Err(MovieSError::NotFound(id.into()))
        }
    }

    pub async fn update_by_id(
        pool: &SqlitePool,
        id: &str,
        up_movie: NMovie,
    ) -> Result<(), MovieSError> {
        let title = up_movie.title;
        let genre = up_movie.genre;
        let release_year = up_movie.release_year;
        Ok(Movie::update_by_id(pool, id, &title, &genre, &release_year)
            .await
            .map_err(|_| MovieSError::Unkown("unable to update".into()))?)
    }

    pub async fn like_movie(
        pool: &SqlitePool,
        user_id: &str,
        movie_id: &str,
    ) -> Result<(), MovieSError> {
        Ok(Movie::like_movie(pool, user_id, movie_id)
            .await
            .map_err(|_| MovieSError::Unkown("Unable to like".into()))?)
    }

    pub async fn unlike_movie(
        pool: &SqlitePool,
        user_id: &str,
        movie_id: &str,
    ) -> Result<(), MovieSError> {
        Ok(Movie::unlike_movie(pool, user_id, movie_id)
            .await
            .map_err(|_| MovieSError::Unkown("Unable to like".into()))?)
    }
}

impl ResponseError for MovieSError {
    fn error_response(&self) -> actix_web::HttpResponse<actix_http::body::BoxBody> {
        log::error!("main error obj. e::: {self:?}");
        match self {
            MovieSError::DbError(_err) => {
                HttpResponse::InternalServerError().body("Internal server1 error")
            }
            MovieSError::NotFound(_err) => HttpResponse::NotFound().body("Not found"),
            MovieSError::AlreadyExists(_e) => HttpResponse::Conflict().body("Already present"),
            _ => {
                log::error!("Unmapped errors:: {self}");
                HttpResponse::BadRequest().body("Something went wrong")
            }
        }
    }
}
