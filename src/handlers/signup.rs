use actix_web::{web, HttpResponse};
use actix_web_flash_messages::{FlashMessage, IncomingFlashMessages};
use maud::{html, Markup};

use crate::{
    domain::{Role, User},
    handlers::{footer, head, header},
};

pub async fn signup_form(messages: IncomingFlashMessages) -> actix_web::Result<Markup> {
    let msgs: Vec<_> = messages.iter().map(|f| f.content()).collect();
    Ok(html! {
        (head("Sign Up"))
        (header())
        div class="content-container" {
            div class="content" {
                h1 class="centered-text job-count-text" {
                    "Sign Up"
                }
                div {
                    @for msg in &msgs {
                        p .error {
                            (msg)
                        }
                    }
                }
                div {
                    form method="POST" action="/signups" class="pure-form pure-form-stacked" {
                        legend { "Sign Up" }
                        label for="employer-cb" class="pure-radio" {
                            input type="radio" id="employer-cb" name="role" value="Employer" checked="" { "Employer "}
                        }
                        label for="dev-cb" class="pure-radio" {
                            input type="radio" id="dev-cb" name="role" value="Dev" checked="" { "Rust Dev"}
                        }
                        label for="email" { "Email" }
                        input type="email" id="email" name="email";
                        label for="password" { "Password" }
                        input type="password" id="password" name="password";
                        label for="repeat_password" { "Repeat password" }
                        input type="password" id="repeat_password" name="repeat_password";
                        button type="submit" class="pure-button pure-button-primary" { "Sign Up" }
                    }
                }
            }
        }
        (footer())
    })
}

#[derive(serde::Deserialize)]
pub struct Signup {
    role: Role,
    email: String,
    password: String,
    repeat_password: String,
}

pub async fn signup(signup: web::Form<Signup>) -> Result<HttpResponse, actix_web::Error> {
    let Signup {
        role,
        email,
        password,
        repeat_password,
    } = signup.0;
    let mut valid_request = true;
    if email.is_empty() {
        valid_request = false;
        FlashMessage::error("Email is mandatory").send();
    }
    if password.is_empty() {
        valid_request = false;
        FlashMessage::error("Password is mandatory").send();
    }
    if password != repeat_password {
        valid_request = false;
        FlashMessage::error("Password doesn't match repeated one").send();
    }
    if !valid_request {
        Ok(HttpResponse::SeeOther()
            .insert_header(("Location", "/signups"))
            .finish())
    } else {
        let u = User {
            email,
            password,
            role,
        };
        log::info!("Fake user {:?} saving ...", u);
        Ok(HttpResponse::SeeOther()
            .insert_header(("Location", "/"))
            .finish())
    }
}
