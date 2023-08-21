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
    handlers::{homepage, job_details, job_search, signup, signup_form},
    monitoring::{liveness, readiness},
};

pub struct Application {
    port: u16,
    server: Server,
}

impl Application {
    pub async fn build(settings: Settings) -> Result<Self, anyhow::Error> {
        let connection_pool = get_connection_pool(&settings.database);
        let address = format!(
            "{}:{}",
            settings.application.host, settings.application.port
        );
        let listener = TcpListener::bind(address)?;
        let port = listener.local_addr().unwrap().port();
        let server = run(
            listener,
            connection_pool,
            settings.application.base_url,
            settings.application.hmac_secret,
            settings.redis_uri,
        )
        .await?;
        Ok(Self { port, server })
    }
    pub fn port(&self) -> u16 {
        self.port
    }
    pub async fn run_until_stopped(self) -> Result<(), std::io::Error> {
        self.server.await
    }
}

pub struct ApplicationBaseUrl(pub String);

async fn run(
    listener: TcpListener,
    db_pool: PgPool,
    base_url: String,
    hmac_secret: Secret<String>,
    redis_uri: Secret<String>,
) -> Result<Server, anyhow::Error> {
    let db_pool = Data::new(db_pool);
    let base_url = Data::new(ApplicationBaseUrl(base_url));
    let secret_key = Key::from(hmac_secret.expose_secret().as_bytes());
    let message_store = CookieMessageStore::builder(secret_key.clone()).build();
    let message_framework = FlashMessagesFramework::builder(message_store).build();
    let redis_store = RedisSessionStore::new(redis_uri.expose_secret()).await?;
    let server = HttpServer::new(move || {
        App::new()
            .wrap(message_framework.clone())
            .wrap(SessionMiddleware::new(
                redis_store.clone(),
                secret_key.clone(),
            ))
            .app_data(Data::new(HmacSecret(hmac_secret.clone())))
            .app_data(db_pool.clone())
            .app_data(base_url.clone())
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
