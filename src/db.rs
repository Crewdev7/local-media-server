use sqlx::{sqlite::SqlitePoolOptions, SqlitePool};

pub async fn init_pool(db_url: &str)->Result<SqlitePool, sqlx::Error>{
    SqlitePoolOptions::new()
        .connect(db_url).await
}


