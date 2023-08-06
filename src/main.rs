mod database;
mod jobs;
mod k8s_probes;
mod presenters;
mod signup;
mod static_files;

use actix_web::{middleware, web, App, HttpServer};
use database::*;
use k8s_probes::*;
use signup::*;
use static_files::*;
use std::sync::Mutex;

use crate::jobs::handlers::job_routes;

#[actix_web::main]
async fn main() {
    env_logger::init();

    let addr = "localhost:8080";

    let db = web::Data::new(Mutex::new(Db::new()));

    let server = HttpServer::new(move || {
        App::new()
            .app_data(db.clone())
            .wrap(middleware::NormalizePath::trim())
            .configure(routes)
    })
    .bind(addr)
    .unwrap()
    .run();

    println!("Server live at http://{}", addr);
    server.await.unwrap();
}

fn routes(cfg: &mut web::ServiceConfig) {
    cfg.configure(static_files)
        .configure(job_routes)
        .configure(k8s_probes)
        .configure(signup);
}
