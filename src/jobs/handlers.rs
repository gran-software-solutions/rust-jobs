use std::sync::Mutex;

use actix_web::{web, Responder, HttpResponse, get};
use sailfish::TemplateOnce;
use uuid::Uuid;

use crate::database::Db;

use super::structs::{Home, JobDetails};

#[get("/")]
async fn homepage(db_mutex: web::Data<Mutex<Db>>) -> impl Responder {
    let db = db_mutex.lock().unwrap();
    HttpResponse::Ok().body(Home {
         jobs: db.get_all(),
         title: "Rust Jobs",
        }.render_once().unwrap())
}

#[get("/jobs/{id}")]
async fn job_details(id: web::Path<Uuid>, db_mutex: web::Data<Mutex<Db>>) -> impl Responder {
    let job_id = id.into_inner();
    let db = db_mutex.lock().unwrap();
    let job = db.get_job(job_id).expect("Expected to find a job");

    HttpResponse::Ok().body(
        JobDetails {
            title: &job.title[..],
            job: job,
        }
        .render_once()
        .unwrap(),
    )
}

pub fn job_routes(cfg: &mut web::ServiceConfig) {
    cfg
    .service(homepage)
    .service(job_details);
}