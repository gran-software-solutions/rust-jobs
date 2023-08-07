use std::sync::Mutex;

use actix_web::{
    get, post,
    web::{self, to, Redirect},
    HttpResponse, Responder,
};
use sailfish::TemplateOnce;
use serde::Deserialize;
use uuid::Uuid;

use crate::{database::Db, jobs::domain::Employer};

#[derive(TemplateOnce)]
#[template(path = "signup.stpl")]
struct SignupTemplate {
    title: String,
    signedup: bool,
    error: bool,
}

#[derive(TemplateOnce)]
#[template(path = "login.stpl")]
struct LoginTemplate {
    title: String,
}

#[derive(Deserialize, Debug)]
pub struct NewEmployer {
    pub name: String,
    pub email: String,
    pub password: String,
}

impl From<NewEmployer> for Employer {
    fn from(new_employer: NewEmployer) -> Self {
        Employer {
            id: Uuid::new_v4().to_string(),
            name: new_employer.name,
            email: new_employer.email,
            password: new_employer.password,
        }
    }
}

#[post("/signup_employer")]
async fn process_signup_form(
    db: web::Data<Mutex<Db>>,
    new_employer: Result<web::Form<NewEmployer>, actix_web::Error>,
) -> impl Responder {
    return match new_employer {
        Ok(data) => {
            db.lock()
                .unwrap()
                .add_employer(Employer::from(data.into_inner()));

            HttpResponse::Ok().body(
                SignupTemplate {
                    error: false,
                    signedup: true,
                    title: "Sign up".to_string(),
                }
                .render_once()
                .unwrap(),
            )
        }
        Err(e) => {
            println!("{:?}", e);
            HttpResponse::SeeOther()
            .append_header(("location", "/signup"))
            .append_header(("X-Error", "true")) // TODO read this in /signup
            .finish()
        }
    };
}

#[get("/signup")]
async fn signup_form() -> impl Responder {
    HttpResponse::Ok().body(
        SignupTemplate {
            error: false,
            signedup: false,
            title: "Sign up".to_string(),
        }
        .render_once()
        .unwrap(),
    )
}

#[get("/login")]
async fn login_form() -> impl Responder {
    HttpResponse::Ok().body(
        LoginTemplate {
            title: String::from("Login"),
        }
        .render_once()
        .unwrap(),
    )
}

#[post("/login")]
async fn process_login_form() -> impl Responder {
    Redirect::to("/").see_other()
}

pub fn signup(cfg: &mut web::ServiceConfig) {
    cfg.service(signup_form)
        .service(process_signup_form)
        .service(login_form)
        .service(process_login_form);
}
