use actix_web::{get, HttpResponse, Responder};

#[get("/liveness")]
async fn liveness() -> impl Responder {
    HttpResponse::Ok()
}

#[get("/readiness")]
async fn readiness() -> impl Responder {
    HttpResponse::Ok()
}
