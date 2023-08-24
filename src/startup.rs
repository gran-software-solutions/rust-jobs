use std::net::TcpListener;

use actix_files::Files;
use actix_session::{storage::RedisSessionStore, SessionMiddleware};
use actix_web::{
    cookie::Key,
    dev::Server,
    middleware,
    web::{self, Data},
    App, HttpServer,
};
use actix_web_flash_messages::{storage::CookieMessageStore, FlashMessagesFramework};
use actix_web_lab::middleware::from_fn;
use anyhow::Ok;
use secrecy::{ExposeSecret, Secret};
use sqlx::{postgres::PgPoolOptions, PgPool};

use crate::{
    authentication::reject_anonymous_users,
    configuration::{DatabaseSettings, Settings},
    database::Database,
    handlers::{homepage, job_details, job_search, signup, signup_form},
    monitoring::{liveness, readiness},
};

pub struct Application {
    pub server: Server,
}

impl Application {
    pub async fn build_and_run(settings: Settings) -> Result<Self, anyhow::Error> {
        let connection_pool = get_connection_pool(&settings.database);
        let address = format!(
            "{}:{}",
            settings.application.host, settings.application.port
        );
        let listener = TcpListener::bind(address)?;
        let server = run_server(
            listener,
            connection_pool,
            settings.application.hmac_secret,
            settings.redis_uri,
        )
        .await?;
        Ok(Self { server })
    }
}

async fn run_server(
    listener: TcpListener,
    db_pool: PgPool,
    hmac_secret: Secret<String>,
    redis_uri: Secret<String>,
) -> anyhow::Result<Server> {
    let db_pool = Data::new(db_pool);
    let secret_key = Key::from(hmac_secret.expose_secret().as_bytes());
    let message_store = CookieMessageStore::builder(secret_key.clone()).build();
    let message_framework = FlashMessagesFramework::builder(message_store).build();
    let redis_store = RedisSessionStore::new(redis_uri.expose_secret()).await?;
    let database = Data::new(Database::new());
    let server = HttpServer::new(move || {
        App::new()
            .app_data(Data::new(HmacSecret(hmac_secret.clone())))
            .app_data(database.clone())
            .app_data(db_pool.clone())
            .wrap(message_framework.clone())
            .wrap(SessionMiddleware::new(
                redis_store.clone(),
                secret_key.clone(),
            ))
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
    .listen(listener)?
    .run();
    Ok(server)
}

pub fn get_connection_pool(configuration: &DatabaseSettings) -> PgPool {
    PgPoolOptions::new()
        .acquire_timeout(std::time::Duration::from_secs(2))
        .connect_lazy_with(configuration.with_db())
}

#[derive(Clone)]
pub struct HmacSecret(pub Secret<String>);
