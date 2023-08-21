mod authentication;
mod configuration;
mod database;
mod domain;
mod handlers;
mod monitoring;
mod session_state;
mod startup;
mod utils;
use crate::authentication::reject_anonymous_users;
use crate::handlers::{homepage, job_details, job_search, signup, signup_form};
use actix_files::Files;
use actix_web::{cookie::Key, middleware, web, App, HttpServer};
use actix_web_flash_messages::{storage::CookieMessageStore, FlashMessagesFramework};
use actix_web_lab::middleware::from_fn;
use configuration::get_configuration;
use database::Database;
use env_logger::Env;
use monitoring::*;
use sqlx::postgres::PgPoolOptions;

#[actix_web::main]
async fn main() {
    env_logger::init_from_env(Env::default().default_filter_or("info"));
    let config = get_configuration().expect("Could not load config");
    let addr = format!("0.0.0.0:{}", config.application.port);

    let pool = match PgPoolOptions::new().connect("url").await {
        Ok(pool) => pool,
        Err(e) => panic!("Failed to apply DB migrations: {}", e),
    };

    let migration_result = sqlx::migrate!("./migrations");
    migration_result
        .run(&pool)
        .await
        .expect("Should have applied migrations");

    let pool = web::Data::new(pool);

    let db = web::Data::new(Database::new());
    let signing_key = Key::generate();
    let message_store = CookieMessageStore::builder(signing_key).build();
    let message_framework = FlashMessagesFramework::builder(message_store).build();

    let server = HttpServer::new(move || {
        App::new()
            .wrap(message_framework.clone())
            .app_data(db.clone())
            .app_data(pool.clone())
            .wrap(middleware::NormalizePath::trim())
            .service(Files::new("/static", "./static/root"))
            .service(web::scope("/probe").service(liveness).service(readiness))
            .route("/", web::get().to(homepage))
            .route("/jobs/search", web::get().to(job_search))
            .route(
                "/jobs/{id}",
                web::get()
                    .wrap(from_fn(reject_anonymous_users))
                    .to(job_details),
            )
            .route("/signups", web::get().to(signup_form))
            .route("/signups", web::post().to(signup))
    })
    .bind(addr)
    .unwrap()
    .run();
    //    let scope = web::scope("/probe").service(liveness).service(readiness);

    println!("Server live at http://{}", addr);
    server.await.unwrap();
}
