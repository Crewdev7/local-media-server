use std::{env, io};

use actix_web::{
    middleware,
    web::{self, scope},
    App, HttpServer,
};
use netflix_clone::auth::route::init_auth_routes;
use sqlx::SqlitePool;

//
// Moduels

mod db;
#[actix_web::main]
async fn main() -> io::Result<()> {
    // initializing local env
    dotenv::dotenv().ok();
    // Logging enabled
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));

    // Data Base setting
    let db_url = env::var("DB_URL").expect("Database url must be set.");
    let pool: SqlitePool = db::init_pool(&db_url)
        .await
        .expect("Failed to init db pool");
    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(pool.clone()))
            .wrap(middleware::Logger::default())
            .service(scope("/auth").configure(init_auth_routes))
    })
    .workers(2)
    .bind("localhost:8080")?
    .run()
    .await
}
