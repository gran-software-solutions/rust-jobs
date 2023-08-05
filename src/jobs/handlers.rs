use actix_web::{get, web, HttpResponse, Responder};
use sailfish::TemplateOnce;
use surrealdb::{engine::remote::ws::Client, Surreal};
use uuid::Uuid;

use crate::jobs::structs::Job;

use super::structs::{Home, JobDetails};

#[get("/")]
async fn homepage(db_client: web::Data<Surreal<Client>>) -> impl Responder {
    let jobs: Vec<Job> = db_client.select("job").await.unwrap();
    HttpResponse::Ok().body(
        Home {
            jobs: &jobs,
            title: "Rust Jobs",
        }
        .render_once()
        .unwrap(),
    )
}

#[get("/jobs/{id}")]
async fn job_details(id: web::Path<Uuid>, db_client: web::Data<Surreal<Client>>) -> impl Responder {
    let job_id = id.into_inner();
    let job: Job = db_client.select(("job", job_id.to_string())).await.unwrap(); // TODO: handle this

    HttpResponse::Ok().body(
        JobDetails {
            title: &job.title[..],
            job: &job,
        }
        .render_once()
        .unwrap(),
    )
}

pub fn job_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(homepage).service(job_details);
}
