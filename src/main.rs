mod handlers;
mod k8s_probes;

use crate::handlers::{homepage, signup_employer, signup_rust_dev};
use actix_files::Files;
use actix_web::{middleware, web, App, HttpServer};
use env_logger::Env;
use k8s_probes::*;

#[actix_web::main]
async fn main() {
    env_logger::init_from_env(Env::default().default_filter_or("info"));

    let addr = "0.0.0.0:8080";

    let server = HttpServer::new(move || {
        App::new()
            .wrap(middleware::NormalizePath::trim())
            .service(Files::new("/static", "./static/root"))
            .service(web::scope("/probe").service(liveness).service(readiness))
            .route("/", web::get().to(homepage))
            .route("/signup/dev", web::get().to(signup_rust_dev))
            .route("/signup/employer", web::get().to(signup_employer))
    })
    .bind(addr)
    .unwrap()
    .run();
    //    let scope = web::scope("/probe").service(liveness).service(readiness);

    println!("Server live at http://{}", addr);
    server.await.unwrap();
}
