use actix_web::{web, HttpResponse};
use actix_web_flash_messages::IncomingFlashMessages;
use maud::html;

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

struct Credentials {
    username: String,
    password: String,
}

impl TryFrom<SignInForm> for Credentials {
    type Error = Vec<String>;

    fn try_from(value: SignInForm) -> Result<Self, Self::Error> {
        todo!()
    }
}

#[derive(serde::Deserialize)]
pub struct SignInForm {
    username: String,
    password: String,
}
pub async fn signin(form: web::Form<SignInForm>) -> HttpResponse {
    let maybe_credentials = form.0.try_into();
    see_other("/")
}
