use actix_web::{Responder, HttpResponse, get, web::{self, Redirect}, post};
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

#[derive(TemplateOnce)]
#[template(path = "login.stpl")]
struct LoginTemplate {
    title: String,
}

#[post("/signup")]
async fn process_signup_form() -> impl Responder {
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

#[get("/login")]
async fn login_form() -> impl Responder {
    HttpResponse::Ok()
    .body(
        LoginTemplate { title: String::from("Login") }
        .render_once()
        .unwrap()
    )
}

#[post("/login")]
async fn process_login_form() -> impl Responder {
    Redirect::to("/").see_other()
}

pub fn signup(cfg: &mut web::ServiceConfig) {
    cfg
    .service(signup_form)
    .service(process_signup_form)
    .service(login_form)
    .service(process_login_form)
    ;
}
