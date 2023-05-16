use actix_web::{web, HttpResponse, Result};
use sqlx::{Pool, Sqlite};

async fn change_subscription(
    pool: web::Data<Pool<Sqlite>>,
    user: AuthenticatedUser,
    new_plan: web::Path<String>,
) -> Result<HttpResponse> {
    let allowed_plans = vec!["basic", "pro", "pro_plus_two"];
    if !allowed_plans.contains(&new_plan.as_str()) {
        return Ok(HttpResponse::BadRequest().body("Invalid subscription plan"));
    }

    sqlx::query!(
        "UPDATE users SET subscription_plan = ? WHERE id = ?",
        new_plan.as_str(),
        user.id
    )
    .execute(pool.as_ref())
    .await?;

    Ok(HttpResponse::Ok().finish())
}

use actix_web::{web, HttpResponse, Result};
use sqlx::{Pool, Sqlite};

async fn add_to_favorite(
    pool: web::Data<Pool<Sqlite>>,
    user: AuthenticatedUser,
    movie_id: web::Path<i32>,
) -> Result<HttpResponse> {
    // check if movie exists
    let movie_exists = sqlx::query!("SELECT 1 FROM movies WHERE id = ?", movie_id.as_ref())
        .fetch_optional(pool.as_ref())
        .await?
        .is_some();

    if !movie_exists {
        return Ok(HttpResponse::NotFound().finish());
    }

    // check if movie is already in favorite list
    let favorite_exists = sqlx::query!(
        "SELECT 1 FROM favorite_movies WHERE user_id = ? AND movie_id = ?",
        user.id,
        movie_id.as_ref()
    )
    .fetch_optional(pool.as_ref())
    .await?
    .is_some();

    if favorite_exists {
        return Ok(HttpResponse::BadRequest().body("Movie already in favorite list"));    }

    // add movie to favorite list
    sqlx::query!(
        "INSERT INTO favorite_movies (user_id, movie_id, is_favorite) VALUES (?, ?, 1)",
        user.id,
        movie_id.as_ref()
    )
    .execute(pool.as_ref())
    .await?;

    Ok(HttpResponse::Ok().finish())
}

use actix_multipart::{Field, Multipart};
use actix_web::{web, HttpResponse, Result};
use futures::{StreamExt, TryStreamExt};

async fn upload_media(
    pool: web::Data<Pool<Sqlite>>,
    user: AuthenticatedUser,
    mut payload: Multipart,
) -> Result<HttpResponse> {
    while let Some(item) = payload.try_next().await? {
        let content_type = item.content_disposition().unwrap().get_type().to_string();
        match content_type.as_str() {
            "form-data" => {
                // handle form data fields
            },
            "file" => {
                // handle file upload
                let mut filename = item
                    .content_disposition()
                    .unwrap()
                    .get_filename()
                    .unwrap()
                    .to_string();

                // generate unique filename to avoid conflicts
                let extension = Path::new(&filename)
                    .extension()
                    .unwrap()
                    .to_str()
                    .unwrap();
                let unique_filename = format!("{}-{}.{}", user.id, Utc::now().timestamp(), extension);
                filename = unique_filename.clone();

                // save file to disk
                let mut file = File::create(format!("uploads/{}", unique_filename)).unwrap();
                while let Some(chunk) = item.next().await {
                    let data = chunk.unwrap();
                    file.write_all(&data).unwrap();
                }

                // insert record into database
                sqlx::query!(
                    "INSERT INTO movies (title, video_url, thumbnail_url) VALUES (?, ?, ?)",
                    filename,
                    format!("/uploads/{}", unique_filename),
                    "/thumbnail.jpg"
                )
                .execute(pool.as_ref())
                .await?;
            },
            _ => {},
        }
    }

    Ok(HttpResponse::Ok().finish())
}

use actix_web::{web, HttpResponse, Result};
use sqlx::{Pool, Sqlite};

async fn get_data_usage(
    pool: web::Data<Pool<Sqlite>>,
    user: AuthenticatedUser,
) -> Result<HttpResponse> {
    let user_data = sqlx::query!("SELECT remaining_data FROM users WHERE id = ?", user.id)
        .fetch_one(pool.as_ref())
        .await?;

    Ok(HttpResponse::Ok().json(user_data))
}



use actix_web::{web, HttpResponse, Result};
use sqlx::{Pool, Sqlite};

async fn search_movies(
    pool: web::Data<Pool<Sqlite>>,
    query_params: web::Query<MovieSearchQuery>,
) -> Result<HttpResponse> {
    let mut query = "SELECT * FROM movies WHERE 1 = 1".to_string();
    let mut params: Vec<&str> = Vec::new();

    if let Some(title) = &query_params.title {
        query.push_str(" AND title LIKE ?");
        params.push(&format!("%{}%", title));
    }

    if let Some(genre) = &query_params.genre {
        query.push_str(" AND genre = ?");
        params.push(genre.as_str());
    }

    if let Some(year) = &query_params.year {
        query.push_str(" AND year = ?");
        params.push(year.as_str());
    }

    let movies = sqlx::query_as::<_, Movie>(&query)
        .bind_all(params)
        .fetch_all(pool.as_ref())
        .await?;

    Ok(HttpResponse::Ok().json(movies))
}

struct MovieSearchQuery {
    title: Option<String>,
    genre: Option<String>,
    year: Option<String>,
}

#[derive(Debug, sqlx::FromRow, Serialize)]
struct Movie {
    id: i32,
    title: String,
    description: String,
    genre: String,
    year: i32,
    video_url: String,
    thumbnail_url: String,
}







use actix_web::{web, HttpResponse, Result};
use sqlx::{Pool, Sqlite};

async fn get_recommendations(
    pool: web::Data<Pool<Sqlite>>,
    user: AuthenticatedUser,
) -> Result<HttpResponse> {
    let watch_history = sqlx::query!(
        "SELECT movie_id FROM watch_history WHERE user_id = ?",
        user.id
    )
    .fetch_all(pool.as_ref())
    .await?;

    let favorite_list = sqlx::query!(
        "SELECT movie_id FROM favorite_movies WHERE user_id = ? AND is_favorite = 1",
        user.id
    )
    .fetch_all(pool.as_ref())
    .await?;

    let mut movie_ids: Vec<i32> = Vec::new();
    for record in watch_history {
        movie_ids.push(record.movie_id);
    }
    for record in favorite_list {
        movie_ids.push(record.movie_id);
    }

    // Your recommendation algorithm goes here

    Ok(HttpResponse::Ok().json(movie_ids))
}











use actix_web::{web, HttpResponse, Result};
use sqlx::{Pool, Sqlite};

async fn add_review(
    pool: web::Data<Pool<Sqlite>>,
    user: AuthenticatedUser,
    movie_id: web::Path<i32>,
    form: web::Form<ReviewForm>,
) -> Result<HttpResponse> {
    // check if movie exists
    let movie_exists = sqlx::query!("SELECT 1 FROM movies WHERE id = ?", movie_id.as_ref())
        .fetch_optional(pool.as_ref())
        .await?
        .is_some();

    if !movie_exists {
        return Ok(HttpResponse::NotFound().finish());
    }

    // check if user has already reviewed the movie
    let review_exists = sqlx::query!(
        "SELECT 1 FROM movie_reviews WHERE user_id = ? AND movie_id = ?",
        user.id,
        movie_id.as_ref()
    )
    .fetch_optional(pool.as_ref())
    .await?
    .is_some();

    if review_exists {
        return Ok(HttpResponse::BadRequest().body("User has already reviewed this movie"));
    }

    // add review to database
    sqlx::query!(
        "INSERT INTO movie_reviews (user_id, movie_id, rating, review) VALUES (?, ?, ?, ?)",
        user.id,
        movie_id.as_ref(),
        form.rating,
        form.review.as_ref().map(|s| s.as_str())
    )
    .execute(pool.as_ref())
    .await?;

    Ok(HttpResponse::Ok().finish())
}

struct ReviewForm {
    rating: i32,
    review: Option<String>,
}



















use actix_web::{web, HttpResponse, Result};
use sqlx::{Pool, Sqlite};

async fn update_profile(
    pool: web::Data<Pool<Sqlite>>,
    user: AuthenticatedUser,
    form: web::Form<ProfileUpdateForm>,
) -> Result<HttpResponse> {
    // check if email already exists
    let email_exists = sqlx::query!(
        "SELECT 1 FROM users WHERE email = ? AND id != ?",
        form.email,
        user.id
    )
    .fetch_optional(pool.as_ref())
    .await?
    .is_some();

    if email_exists {
        return Ok(HttpResponse::BadRequest().body("Email already exists"));
    }

    // update user record in database
    sqlx::query!(
        "UPDATE users SET name = ?, email = ? WHERE id = ?",
        form.name,
        form.email,
        user.id
    )
    .execute(pool.as_ref())
    .await?;

    Ok(HttpResponse::Ok().finish())
}

struct ProfileUpdateForm {
    name: String,
    email: String,
    password: Option<String>,
}


















use actix_web::{web, HttpResponse, Result};
use sqlx::{Pool, Sqlite};

async fn get_subscription(
    pool: web::Data<Pool<Sqlite>>,
    user: AuthenticatedUser,
) -> Result<HttpResponse> {
    let subscription = sqlx::query!(
        "SELECT plan, start_date, end_date FROM subscriptions WHERE user_id = ?",
        user.id
    )
    .fetch_optional(pool.as_ref())
    .await?;

    if let Some(subscription) = subscription {
        Ok(HttpResponse::Ok().json(subscription))
    } else {
        Ok(HttpResponse::NotFound().finish())
    }
}

async fn update_subscription(
    pool: web::Data<Pool<Sqlite>>,
    user: AuthenticatedUser,
    form: web::Form<SubscriptionUpdateForm>,
) -> Result<HttpResponse> {
    // check if plan is valid
    let valid_plans = ["basic", "standard", "premium"];
    if !valid_plans.contains(&form.plan.as_str()) {
        return Ok(HttpResponse::BadRequest().body("Invalid plan"));
    }

    // update subscription record in database
    sqlx::query!(
        "UPDATE subscriptions SET plan = ? WHERE user_id = ?",
        form.plan,
        user.id
    )
    .execute(pool.as_ref())
    .await?;

    Ok(HttpResponse::Ok().finish())
}

struct SubscriptionUpdateForm {
    plan: String,
}

// Payment endpoint using Stripe
async fn create_payment_intent(
    pool: web::Data<Pool<Sqlite>>,
    user: AuthenticatedUser,
    form: web::Form<PaymentForm>,
) -> Result<HttpResponse> {
    let amount = form.amount;

    // create payment intent with Stripe
    let intent = stripe::PaymentIntent::create(&stripe::PaymentIntentParams {
        amount: amount,
        currency: "usd".to_string(),
        payment_method_types: vec!["card".to_string()],
        metadata: Some(vec![
            ("user_id".to_string(), user.id.to_string()),
            ("movie_id".to_string(), form.movie_id.to_string()),
        ])
        .into_iter()
        .collect(),
        ..Default::default()
    })
    .await?;

    // save payment intent ID in database
    sqlx::query!(
        "INSERT INTO payment_intents (user_id, movie_id, amount, intent_id) VALUES (?, ?, ?, ?)",
        user.id,
        form.movie_id,
        amount,
        intent.id.as_str()
    )
    .execute(pool.as_ref())
    .await?;

    Ok(HttpResponse::Ok().json(intent))
}

struct PaymentForm {
    movie_id: i32,
    amount: i32,
}
