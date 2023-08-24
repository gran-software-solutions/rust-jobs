mod authentication;
mod configuration;
mod database;
mod domain;
mod handlers;
mod monitoring;
mod session_state;
mod startup;
mod utils;
use crate::startup::Application;

use configuration::get_settings;
use env_logger::Env;
use sqlx::PgPool;
type AnyhowResult = anyhow::Result<()>;

#[actix_web::main]
async fn main() -> AnyhowResult {
    env_logger::init_from_env(Env::default().default_filter_or("info"));
    let settings = get_settings().expect("Could not load config");
    let pg_connect_options = settings.database.without_db();
    let pool = PgPool::connect_with(pg_connect_options)
        .await
        .expect("TODO");

    let migration_result = sqlx::migrate!("./migrations");
    migration_result
        .run(&pool)
        .await
        .expect("Should have applied migrations");

    Application::build_and_run(settings.clone())
        .await?
        .server
        .await?;
    Ok(())
}
