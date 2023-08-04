use crate::database::Db;

use std::sync::Mutex;

use actix_web::{web, Responder, HttpResponse};
use sailfish::TemplateOnce;
use serde::Deserialize;
use uuid::Uuid;


#[derive(Deserialize)]
pub struct Job {
    pub id: String,
    pub title: String,
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

pub async fn homepage(db_mutex: web::Data<Mutex<Db>>) -> impl Responder {
    let db = db_mutex.lock().unwrap();
    HttpResponse::Ok().body(Home { jobs: db.get_all() }.render_once().unwrap())
}

pub async fn job_details(id: web::Path<Uuid>, db_mutex: web::Data<Mutex<Db>>) -> impl Responder {
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