use std::sync::Mutex;

use actix_web::{get, web, HttpResponse, Responder};
use uuid::Uuid;

use crate::{
    database::Db,
    presenters::{home_presenter, job_details_presenter},
};

#[get("/")]
async fn homepage(db_mutex: web::Data<Mutex<Db>>) -> impl Responder {
    let db = db_mutex.lock().unwrap();

    HttpResponse::Ok().body(home_presenter(db.get_all()))
}

#[get("/jobs/{id}")]
async fn job_details(id: web::Path<Uuid>, db_mutex: web::Data<Mutex<Db>>) -> impl Responder {
    let job_id = id.into_inner();
    let db = db_mutex.lock().unwrap();

    let job = db.get_job(job_id).expect("Expected to find a job");

    HttpResponse::Ok().body(job_details_presenter(job))
}

pub fn job_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(homepage).service(job_details);
}
