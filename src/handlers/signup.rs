use maud::{html, Markup};

use crate::handlers::{footer, head, header};

pub async fn signup_rust_dev() -> actix_web::Result<Markup> {
    Ok(html! {
        (head("Sign Up as Rust Dev"))
        (header())
        div class="content-container" {
            div class="content" {
                h1 class="centered-text job-count-text" {
                    "Sign Up as Rust Dev"
                }
                div {
                    form class="pure-form pure-form-stacked" {
                        legend { "Sign Up as Rust Dev" }
                        label for="email" { "Email" }
                        input type="email" id="email" name="email";
                        label for="password" { "Password" }
                        input type="password" id="password" name="password";
                        label for="repeat-password" { "Repeat password" }
                        input type="password" id="repeat-password" name="repeat-password";
                        button type="submit" class="pure-button pure-button-primary" { "Sign Up" }
                    }
                }
            }
        }
        (footer())
    })
}

pub async fn signup_employer() -> actix_web::Result<Markup> {
    Ok(html! {
        (head("Sign Up as Employer"))
        (header())
        div class="content-container" {
            div class="content" {
                h1 class="centered-text job-count-text" {
                    "Sign Up as Employer"
                }
                div {
                    form class="pure-form pure-form-stacked" {
                        legend { "Sign Up as Employer" }
                        label for="email" { "Email" }
                        input type="email" id="email" name="email";
                        label for="password" { "Password" }
                        input type="password" id="password" name="password";
                        label for="repeat-password" { "Repeat password" }
                        input type="password" id="repeat-password" name="repeat-password";
                        button type="submit" class="pure-button pure-button-primary" { "Sign Up" }
                    }
                }
            }
        }
        (footer())
    })
}
