use std::io::Result;
use actix_web::{HttpServer, App, web};
use rust_jobs::hello;

#[actix_web::main]
async fn main() -> Result<()> {
    HttpServer::new(|| {
        App::new()
        .route("/", web::get().to(hello))
    })
    .bind(("127.0.0.1", 8888))?
    .run()
    .await
}
