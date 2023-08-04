use actix_files::Files;
use actix_web::web;

pub fn static_files(cfg: &mut web::ServiceConfig) {
    cfg
    .service(Files::new("/static", "./static/root/"));
}
