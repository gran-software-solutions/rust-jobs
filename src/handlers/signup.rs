use actix_web::web;
use maud::{html, Markup};

use crate::handlers::{footer, head, header};

pub async fn signup_form() -> actix_web::Result<Markup> {
    Ok(html! {
        (head("Sign Up"))
        (header())
        div class="content-container" {
            div class="content" {
                h1 class="centered-text job-count-text" {
                    "Sign Up"
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
    email: String,
    password: String,
    repeat_password: String,
}

pub async fn signup(signup: web::Form<Signup>) -> actix_web::Result<Markup> {
    let Signup {
        email,
        password,
        repeat_password,
    } = signup.0;
    Ok(html! {
        (head("Sign Up"))
        (header())
        div class="content-container" {
            div class="content" {
                h1 class="centered-text job-count-text" {
                    "Sign Up"
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
