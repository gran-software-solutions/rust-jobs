use std::sync::Mutex;

use actix_web::{
    get,
    http::StatusCode,
    post,
    web::{self, Redirect},
    HttpResponse, Responder,
};
use serde::Deserialize;
use uuid::Uuid;

use crate::{
    database::Db,
    jobs::domain::Job,
    presenters::{home_presenter, job_details_presenter, new_job_presenter},
};

use super::domain::{JobType, Location};

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

#[get("/jobs/new")]
async fn new_job_view() -> impl Responder {
    HttpResponse::Ok().body(new_job_presenter())
}

#[derive(Deserialize)]
struct NewJob {
    title: String,
    job_type: JobType,
    start: String,
    location: Location,
}

#[post("/jobs")]
async fn save_new_job(
    new_job: web::Form<NewJob>,
    db_mutex: web::Data<Mutex<Db>>,
) -> impl Responder {
    let new_job = new_job.into_inner();
    db_mutex.lock().unwrap().add_job(Job::new(
        new_job.title,
        new_job.start,
        new_job.job_type,
        new_job.location,
    ));
    Redirect::to("/").using_status_code(StatusCode::SEE_OTHER)
}

pub fn job_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(homepage)
        .service(new_job_view)
        .service(save_new_job)
        .service(job_details);
}
