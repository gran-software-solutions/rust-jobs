mod jobs;

use actix_web::{web, App, HttpServer};
use jobs::handlers::{homepage, job_details};

mod database;
use database::*;
use std::sync::Mutex;

#[actix_web::main]
async fn main() {
    let addr = "localhost:8080";

    let db = web::Data::new(Mutex::new(Db::new()));

    let server = HttpServer::new(move || {
        App::new()
            .app_data(db.clone())
            .route("/", web::get().to(homepage))
            .route("/jobs/{job_id}", web::get().to(job_details))
    })
    .bind(addr)
    .unwrap()
    .run();

    println!("Server live at http://{}", addr);
    server.await.unwrap();
}
