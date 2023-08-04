mod jobs;
mod database;
mod static_files;

use actix_web::{web, App, HttpServer, middleware};
use database::*;
use static_files::*;
use std::sync::Mutex;

use crate::jobs::handlers::job_routes;

#[actix_web::main]
async fn main() {
    let addr = "localhost:8080";

    let db = web::Data::new(Mutex::new(Db::new()));

    let server = HttpServer::new(move || {
        App::new()
            .app_data(db.clone())
            .wrap(middleware::NormalizePath::trim())
            .configure(job_routes)
            .configure(static_files)
    })
    .bind(addr)
    .unwrap()
    .run();

    println!("Server live at http://{}", addr);
    server.await.unwrap();
}
