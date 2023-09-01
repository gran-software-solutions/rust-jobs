use actix_web::{web, HttpResponse};
use actix_web_flash_messages::IncomingFlashMessages;
use maud::html;
use serde::Deserialize;
use validator::{Validate, ValidationErrors};

use crate::{
    handlers::{footer, head, header},
    utils::see_other,
};

pub async fn sign_in_view(messages: IncomingFlashMessages) -> HttpResponse {
    let msgs: Vec<_> = messages.iter().map(|f| f.content()).collect();
    let pre_escaped = html! {
        (head("Sign In"))
        (header())
        div class="content-container" {
            div class="content" {
                h1 class="centered-text job-count-text" {
                    "Sign In"
                }
                div {
                    @for msg in &msgs {
                        p .error {
                            (msg)
                        }
                    }
                }
                div {
                    form method="POST" action="/signin" class="pure-form pure-form-stacked" {
                        legend { "Sign In" }
                        label for="email" { "Email" }
                        input type="email" id="email" name="email";
                        label for="password" { "Password" }
                        input type="password" id="password" name="password";
                        button type="submit" class="pure-button pure-button-primary" { "Sign In" }
                    }
                }
            }
        }
        (footer())
    };
    HttpResponse::Ok().body(pre_escaped.into_string())
}

#[derive(Debug, Validate, Deserialize)]
struct NewSignIn {
    #[validate(email(message = "Invalid email"))]
    email: String,
    #[validate(length(min = 8, message = "Password must be at least 8 chars long"))]
    password: String,
}

impl TryFrom<SignInForm> for NewSignIn {
    type Error = ValidationErrors;

    fn try_from(value: SignInForm) -> Result<Self, Self::Error> {
        let new_sign_in = NewSignIn {
            email: value.username,
            password: value.password,
        };
        new_sign_in.validate()?;
        Ok(new_sign_in)
    }
}

#[derive(serde::Deserialize)]
pub struct SignInForm {
    username: String,
    password: String,
}
pub async fn signin(form: web::Form<SignInForm>) -> HttpResponse {
    let new_sign_in: Result<NewSignIn, ValidationErrors> = form.0.try_into();
    let new_sign_in = match new_sign_in {
        Ok(n) => n,
        Err(_errors) => return see_other("/siginin"),
    };
    match validate_credentials(new_sign_in).await {
        Ok(()) => {
            //  TODO:
            see_other("/")
        }
        Err(e) => {
            // TODO:
            see_other("/signin")
        }
    }
}

async fn validate_credentials(n: NewSignIn) -> Result<(), anyhow::Error> {
    // TODO:
    Result::Ok(())
}
