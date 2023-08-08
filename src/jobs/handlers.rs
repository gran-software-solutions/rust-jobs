use std::sync::Mutex;

use actix_web::{
    get, post,
    web::{self, Redirect},
    HttpResponse, Responder,
};
use log::info;
use serde::Deserialize;
use uuid::Uuid;

use crate::jobs::domain::{Currency, JobLocation, JobType, Rate, RateTimeUnit};
use crate::presenters::freelance_job_presenter;
use crate::{
    database::Db,
    jobs::domain::FreelanceJob,
    presenters::{home_presenter, new_regular_job_presenter, not_found, regular_job_presenter},
};

#[get("/")]
async fn homepage(db_mutex: web::Data<Mutex<Db>>) -> impl Responder {
    let db = db_mutex.lock().unwrap();

    HttpResponse::Ok().body(home_presenter(db.get_all()))
}

#[get("/freelance-jobs/{id}")]
async fn freelance_job_details(
    id: web::Path<Uuid>,
    db_mutex: web::Data<Mutex<Db>>,
) -> impl Responder {
    let job_id = id.into_inner();
    let db = db_mutex.lock().unwrap();

    match db.get_freelance_job(job_id) {
        None => {
            HttpResponse::NotFound().body(not_found(format!("Job with id {} not found", job_id)))
        }
        Some(job) => HttpResponse::Ok().body(freelance_job_presenter(job)),
    }
}

#[get("/regular-jobs/{id}")]
async fn regular_job_details(
    id: web::Path<Uuid>,
    db_mutex: web::Data<Mutex<Db>>,
) -> impl Responder {
    let job_id = id.into_inner();
    let db = db_mutex.lock().unwrap();

    match db.get_regular_job(job_id) {
        None => {
            HttpResponse::NotFound().body(not_found(format!("Job with id {} not found", job_id)))
        }
        Some(job) => HttpResponse::Ok().body(regular_job_presenter(job)),
    }
}

#[get("/freelance-jobs/new")]
async fn new_freelance_job_view() -> impl Responder {
    HttpResponse::Ok().body(new_regular_job_presenter())
}

#[get("/regular-jobs/new")]
async fn new_regular_job_view() -> impl Responder {
    HttpResponse::Ok().body(new_regular_job_presenter())
}

#[derive(Deserialize)]
struct NewFreelanceJob {
    title: String,
    start: Option<String>,
    duration_in_months: u16,
    rate: u16,
    rate_currency: Currency,
    rate_time_unit: RateTimeUnit,
    hours_per_week: u8,
    location: JobLocation,
    office_location: Option<String>,
    description: String,
}

impl From<NewFreelanceJob> for FreelanceJob {
    fn from(n: NewFreelanceJob) -> Self {
        Self {
            id: Uuid::new_v4().to_string(),
            employer: "GRAN GmbH".to_string(), // todo
            duration_in_months: n.duration_in_months,
            description: n.description,
            hours_per_week: n.hours_per_week,
            office_location: n.office_location,
            rate: Rate::new(n.rate, n.rate_currency, n.rate_time_unit),
            title: n.title,
            start: n.start,
            location: n.location,
        }
    }
}

#[post("/freelance-jobs")]
async fn save_new_freelance_job(
    new_job: web::Form<NewFreelanceJob>,
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
        .service(new_regular_job_view)
        .service(save_new_freelance_job)
        .service(mutate_job)
        .service(freelance_job_details)
        .service(regular_job_details);
}
