use actix_web::{get, web, HttpResponse, Responder};
use sailfish::TemplateOnce;
use surrealdb::{engine::remote::ws::Client, Surreal};

use crate::{database::SurrealJob, jobs::structs::Job};

use super::structs::{Home, JobDetails};

#[get("/")]
async fn homepage(db_client: web::Data<Surreal<Client>>) -> impl Responder {
    let jobs: Vec<SurrealJob> = db_client.select("job").await.unwrap();
    HttpResponse::Ok().body(
        Home {
            jobs: &jobs
                .iter()
                .map(|j| Job {
                    id: j.id.id.to_string(),
                    title: j.title.clone(),
                })
                .collect(),
            title: "Rust Jobs",
        }
        .render_once()
        .unwrap(),
    )
}

#[get("/jobs/{id}")]
async fn job_details(
    id: web::Path<String>,
    db_client: web::Data<Surreal<Client>>,
) -> impl Responder {
    let db_job: SurrealJob = db_client
        .select(("job", dbg!(id.into_inner())))
        .await
        .unwrap(); // TODO: handle this

    HttpResponse::Ok().body(
        JobDetails {
            title: &db_job.title.clone(),
            job: &Job {
                id: db_job.id.id.to_string(),
                title: db_job.title,
            },
        }
        .render_once()
        .unwrap(),
    )
}

pub fn job_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(homepage).service(job_details);
}
