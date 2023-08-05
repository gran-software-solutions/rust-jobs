mod database;
mod jobs;
mod k8s_probes;
mod signup;
mod static_files;

use std::io::{Error, ErrorKind};

use actix_web::{middleware, web, App, HttpServer};
use database::*;
use k8s_probes::*;
use log::error;
use signup::*;
use static_files::*;
use surrealdb::{engine::remote::ws::Ws, opt::auth::Database, Surreal};

use crate::jobs::handlers::job_routes;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init();

    let addr = "localhost:8080";

    let db_conf = SurrealdbConfig::new();

    let db_client = match Surreal::new::<Ws>(format!("{}:{}", db_conf.host, db_conf.port)).await {
        Ok(client) => client,
        Err(e) => {
            error!("Could not initialize db client {}", e);
            return Err(Error::new(ErrorKind::Other, "Could not connect to db!"));
        }
    };
    db_client.signin(Database {
        namespace: &db_conf.namespace,
        database: &db_conf.database,
        username: &db_conf.username,
        password: &db_conf.password,
    });
    db_client
        .use_ns(&db_conf.namespace)
        .use_db(&db_conf.database);

    let server = HttpServer::new(move || {
        App::new()
            .app_data(db_client.clone())
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
