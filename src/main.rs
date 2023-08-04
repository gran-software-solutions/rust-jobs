mod jobs;

use actix_web::{web, App, HttpServer, middleware};

mod database;
use database::*;
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
    })
    .bind(addr)
    .unwrap()
    .run();

    println!("Server live at http://{}", addr);
    server.await.unwrap();
}
