use actix_web::{Responder, HttpResponse, get, web};
use sailfish::TemplateOnce;

#[derive(TemplateOnce)]
#[template(path = "signup.stpl")]
struct SignupTemplate {
    title: String,
}

impl SignupTemplate {
    fn new(title: &str) -> Self {
        Self { title: String::from(title) }
    }
}

#[get("/signup")]
async fn signup_form() -> impl Responder {
    HttpResponse::Ok()
    .body(SignupTemplate::new("Sign Up!").render_once().unwrap())
}

pub fn signup(cfg: &mut web::ServiceConfig) {
    cfg.service(signup_form);
}
