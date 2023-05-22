use serde::{Deserialize, Serialize};
use sqlx::{query, query_as, FromRow, SqlitePool};

#[derive(Debug, FromRow, Serialize, Deserialize)]
pub struct Movie {
    pub id: i64,
    pub title: String,
    pub genre: String,
    pub release_year: i64,
    pub created: String,
}
type SError<T> = sqlx::Result<T>;

impl Movie {
    pub async fn create(
        pool: &SqlitePool,
        title: &str,
        genre: &str,
        releas_year: &str,
    ) -> SError<i32> {
        let insert_result = sqlx::query!(
            "INSERT INTO movies  ( title,genre,release_year) VALUES (?, ?, ?)",
            title,
            genre,
            releas_year
        )
        .execute(pool)
        .await?;

        Ok(insert_result.last_insert_rowid() as i32)
    }

    pub async fn find_by_id(pool: &SqlitePool, id: &str) -> SError<Option<Movie>> {
        let user = query_as!(Movie, r#"SELECT * FROM movies  WHERE  id= ?"#, id)
            .fetch_optional(pool)
            .await?;
        Ok(user)
    }
    // Todo  without genreword or map it  to remove  it or  create new struct for this one;
    pub async fn get_all(pool: &SqlitePool) -> SError<Vec<Movie>> {
        Ok(query_as!(Movie, "SELECT *  FROM  movies ")
            .fetch_all(pool)
            .await?)
    }

    pub async fn get_all_liked_movie(pool: &SqlitePool, user_id: &str) -> SError<Vec<Movie>> {
        Ok(query_as!(
            Movie,
            "SELECT movies.* FROM movies
        INNER JOIN likes ON movies.id = likes.movie_id
        WHERE likes.user_id = ?",
            user_id
        )
        .fetch_all(pool)
        .await?)
    }

    pub async fn delete_by_id(pool: &SqlitePool, id: &str) -> SError<u64> {
        let res = query!("DELETE  FROM  movies  WHERE id = ?", id)
            .execute(pool)
            .await?;
        Ok(res.rows_affected())
    }

    pub async fn update_movie(
        &self,
        pool: &SqlitePool,
        new_title: &str,
        new_genre: &str,
        new_releas_year: &str,
    ) -> SError<()> {
        sqlx::query!(
            r#"UPDATE movies  SET (title,genre,release_year) = (?,?,?) WHERE id = ?"#,
            new_title,
            new_genre,
            new_releas_year,
            self.id
        )
        .execute(pool)
        .await?;
        Ok(())
    }

    pub async fn update_by_id(
        pool: &SqlitePool,
        id: &str,
        new_title: &str,
        new_genre: &str,
        new_releas_year: &str,
    ) -> SError<()> {
        sqlx::query!(
            r#"UPDATE movies  SET (title,genre,release_year) = (?,?,?) WHERE id = ?"#,
            new_title,
            new_genre,
            new_releas_year,
            id
        )
        .execute(pool)
        .await?;
        Ok(())
    }

    pub async fn like_movie(
        pool: &SqlitePool,
        user_id: &str,
        movie_id: &str,
    ) -> Result<(), sqlx::Error> {
        sqlx::query("INSERT INTO likes (user_id, movie_id) VALUES (?, ?) ON CONFLICT DO NOTHING")
            .bind(user_id)
            .bind(movie_id)
            .execute(pool)
            .await?;
        Ok(())
    }

    pub async fn unlike_movie(
        pool: &SqlitePool,
        user_id: &str,
        movie_id: &str,
    ) -> Result<(), sqlx::Error> {
        sqlx::query("DELETE FROM  likes WHERE user_id = ?  AND movie_id= ?")
            .bind(user_id)
            .bind(movie_id)
            .execute(pool)
            .await?;
        Ok(())
    }
}
