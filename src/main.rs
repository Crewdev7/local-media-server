use std::{env, io};

use actix_identity::IdentityMiddleware;
use actix_session::{config::PersistentSession, storage::CookieSessionStore, SessionMiddleware};
use actix_web::{
    cookie::{time::Duration, Key},
    middleware,
    web::{self, scope},
    App, HttpServer,
};
use netflix_clone::api::{auth_routes::init_auth_route, user_routes::init_user_route};
use sqlx::SqlitePool;

//
// Moduels

mod db;
#[actix_web::main]
async fn main() -> io::Result<()> {
    // initializing local env
    dotenv::dotenv().ok();
    // Logging enabled
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("debug"));

    let secret_key = Key::derive_from(Key::generate().master());

    let db_url = env::var("DB_URL").expect("Database url must be set.");
    let pool: SqlitePool = db::init_pool(&db_url)
        .await
        .expect("Failed to init db pool");

    HttpServer::new(move || {
        let session = SessionMiddleware::builder(CookieSessionStore::default(), secret_key.clone())
            .cookie_name("testing_c".into())
            .cookie_secure(false)
            .session_lifecycle(PersistentSession::default().session_ttl(Duration::days(1)))
            .build();

        App::new()
            .wrap(middleware::NormalizePath::trim())
            .app_data(web::Data::new(pool.clone()))
            .wrap(IdentityMiddleware::default())
            .wrap(session)
            .wrap(middleware::Logger::default())
            .service(
                scope("/api")
                    .configure(init_user_route)
                    .configure(init_auth_route),
            )
    })
    .workers(2)
    .bind("localhost:8080")?
    .run()
    .await
}
