use actix_web::{Responder, HttpResponse, get, web, post};
use sailfish::TemplateOnce;

#[derive(TemplateOnce)]
#[template(path = "signup.stpl")]
struct SignupTemplate {
    title: String,
    signedup: bool,
}

impl SignupTemplate {
    fn new(title: &str, signedup: bool) -> Self {
        Self {
            title: String::from(title),
            signedup
        }
    }
}

#[post("/signup")]
async fn signup_process_form() -> impl Responder {
    HttpResponse::Ok()
    .body(
        SignupTemplate::new("Sign Up!", true)
        .render_once()
        .unwrap()
    )
}

#[get("/signup")]
async fn signup_form() -> impl Responder {
    HttpResponse::Ok()
    .body(
        SignupTemplate::new("Sign Up!", false)
        .render_once()
        .unwrap()
    )
}

pub fn signup(cfg: &mut web::ServiceConfig) {
    cfg
    .service(signup_form)
    .service(signup_process_form)
    ;
}
