mod database;
mod jobs;
mod k8s_probes;
mod signup;
mod static_files;

use actix_web::web;
use database::*;
use k8s_probes::*;
use signup::*;
use static_files::*;
use surrealdb::{engine::remote::ws::Ws, Surreal, opt::auth::Root};
use std::env;

use crate::jobs::handlers::job_routes;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let addr = "localhost:8080";

    let surrealdb_config = SurrealdbConfig::new();


    let storage = match Surreal::new::<Ws>(format!("{}:{}", surrealdb_config.host, surrealdb_config.port)).await {
        Ok(client) => client,
        Err(e) => return Err(std::io::Error::new(std::io::ErrorKind::Other, "Could not connect to db!")),
    };
    storage.signin(Root{
        username: &surrealdb_config.username,
        password: &surrealdb_config.password,
    });

    if let Ok(ns) = env::var("SURREALDB_NAMESPACE") {
        storage.use_ns(ns);
    }

    storage.use_db(surrealdb_config.database);


    let db = web::Data::new(Mutex::new(Db::new()));

    let server = HttpServer::new(move || {
        App::new()
            .app_data(db.clone())
            .wrap(middleware::NormalizePath::trim())
            .configure(routes)
    })
    .bind(addr)
    .unwrap()
    .run();

    println!("Server live at http://{}", addr);
    server.await.unwrap();
    Ok(())
}

fn routes(cfg: &mut web::ServiceConfig) {
    cfg.configure(static_files)
        .configure(job_routes)
        .configure(k8s_probes)
        .configure(signup);
}
