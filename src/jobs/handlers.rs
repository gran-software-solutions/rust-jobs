use std::sync::Mutex;

use actix_web::{
    get, post,
    web::{self, Redirect},
    HttpResponse, Responder,
};
use serde::Deserialize;
use uuid::Uuid;

use crate::{
    database::Db,
    jobs::domain::Job,
    presenters::{home_presenter, job_details_presenter, new_job_presenter, not_found},
};

use super::domain::{JobType, Location};
use log::info;

#[get("/")]
async fn homepage(db_mutex: web::Data<Mutex<Db>>) -> impl Responder {
    let db = db_mutex.lock().unwrap();

    HttpResponse::Ok().body(home_presenter(db.get_all()))
}

#[get("/jobs/{id}")]
async fn job_details(id: web::Path<Uuid>, db_mutex: web::Data<Mutex<Db>>) -> impl Responder {
    let job_id = id.into_inner();
    let db = db_mutex.lock().unwrap();

    match db.get_job(job_id) {
        None => {
            HttpResponse::NotFound().body(not_found(format!("Job with id {} not found", job_id)))
        }
        Some(job) => HttpResponse::Ok().body(job_details_presenter(job)),
    }
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
    employer: String,
}

impl From<NewJob> for Job {
    fn from(n: NewJob) -> Self {
        Self {
            id: Uuid::new_v4().to_string(),
            employer: n.employer,
            job_type: n.job_type,
            title: n.title,
            start: n.start,
            location: n.location,
        }
    }
}

#[post("/jobs")]
async fn save_new_job(
    new_job: web::Form<NewJob>,
    db_mutex: web::Data<Mutex<Db>>,
) -> impl Responder {
    let new_job = new_job.into_inner();
    db_mutex.lock().unwrap().add_job(new_job.into());
    Redirect::to("/").see_other()
}

#[derive(PartialEq, Deserialize)]
enum JobMutation {
    DELETE,
    UPDATE,
}

#[derive(Deserialize)]
struct JobMutationForm {
    job_mutation: JobMutation,
}

#[post("/jobs/{id}")]
async fn mutate_job(
    form: web::Form<JobMutationForm>,
    id: web::Path<Uuid>,
    db_mutex: web::Data<Mutex<Db>>,
) -> impl Responder {
    let job_id = id.into_inner();
    if form.job_mutation == JobMutation::DELETE {
        info!("Deleting job {}", job_id);
        db_mutex.lock().unwrap().delete(job_id);
        Redirect::to("/").see_other()
    } else {
        todo!("Mutation not supported")
    }
}

pub fn job_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(homepage)
        .service(new_job_view)
        .service(save_new_job)
        .service(mutate_job)
        .service(job_details);
}
