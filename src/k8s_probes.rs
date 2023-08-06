use actix_web::{get, web, HttpResponse, Responder};

#[get("/liveness")]
async fn liveness() -> impl Responder {
    HttpResponse::Ok()
}

#[get("/readiness")]
async fn readiness() -> impl Responder {
    HttpResponse::Ok()
}

pub fn k8s_probes(cfg: &mut web::ServiceConfig) {
    let scope = web::scope("/probe").service(liveness).service(readiness);

    cfg.service(scope);
}
