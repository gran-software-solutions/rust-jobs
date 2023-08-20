mod database;
mod domain;
mod handlers;
mod k8s_probes;
use crate::handlers::{homepage, job_details, job_search, signup, signup_form};
use actix_files::Files;
use actix_web::{middleware, web, App, HttpServer};
use database::Database;
use env_logger::Env;
use k8s_probes::*;

#[actix_web::main]
async fn main() {
    env_logger::init_from_env(Env::default().default_filter_or("info"));

    let addr = "0.0.0.0:8080";

    let db = web::Data::new(Database::new());

    let server = HttpServer::new(move || {
        App::new()
            .app_data(db.clone())
            .wrap(middleware::NormalizePath::trim())
            .service(Files::new("/static", "./static/root"))
            .service(web::scope("/probe").service(liveness).service(readiness))
            .route("/", web::get().to(homepage))
            .route("/jobs/search", web::get().to(job_search))
            .route("/jobs/{id}", web::get().to(job_details))
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
