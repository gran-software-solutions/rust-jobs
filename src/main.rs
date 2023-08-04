use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use sailfish::TemplateOnce;
use serde::Deserialize;

mod database;
use database::*;
use std::sync::Mutex;
use uuid::Uuid;

#[derive(Deserialize)]
pub struct Job {
    id: String,
    title: String,
}

#[derive(TemplateOnce)]
#[template(path = "home.stpl")]
struct Home<'a> {
    jobs: &'a Vec<Job>,
}

#[derive(TemplateOnce)]
#[template(path = "job.stpl")]
struct JobDetails<'a> {
    job: &'a Job,
}

async fn homepage(db_mutex: web::Data<Mutex<Db>>) -> impl Responder {
    let db = db_mutex.lock().unwrap();
    HttpResponse::Ok().body(Home { jobs: db.get_all() }.render_once().unwrap())
}
async fn job_details(id: web::Path<Uuid>, db_mutex: web::Data<Mutex<Db>>) -> impl Responder {
    let job_id = id.into_inner();
    let db = db_mutex.lock().unwrap();
    HttpResponse::Ok().body(
        JobDetails {
            job: db.get_job(job_id).expect("Expected to find a job"),
        }
        .render_once()
        .unwrap(),
    )
}

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
