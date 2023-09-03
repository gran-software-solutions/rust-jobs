use actix_web::{rt::task::spawn_blocking, web, HttpResponse};
use actix_web_flash_messages::{FlashMessage, IncomingFlashMessages, Level};
use anyhow::Context;
use maud::html;
use secrecy::Secret;
use serde::Deserialize;
use sqlx::PgPool;
use uuid::Uuid;
use validator::{Validate, ValidationErrors};

use crate::{
    authentication::verify_password_hash,
    handlers::{footer, head, header},
    session_state::TypedSession,
    utils::see_other,
};

#[derive(thiserror::Error, Debug)]
pub enum AuthError {
    #[error("Invalid credentials.")]
    InvalidCredentials(#[source] anyhow::Error),
    #[error(transparent)]
    UnexpectedError(#[from] anyhow::Error),
}

pub async fn sign_in_view(messages: IncomingFlashMessages, session: TypedSession) -> HttpResponse {
    let msgs: Vec<_> = messages.iter().map(|f| f.content()).collect();
    let pre_escaped = html! {
        (head("Sign In"))
        (header(session.get_user_id().unwrap().is_some()))
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
            email: value.email,
            password: value.password,
        };
        new_sign_in.validate()?;
        Ok(new_sign_in)
    }
}

#[derive(serde::Deserialize)]
pub struct SignInForm {
    email: String,
    password: String,
}
pub async fn signin(
    form: web::Form<SignInForm>,
    pool: web::Data<PgPool>,
    session: TypedSession,
) -> HttpResponse {
    let new_sign_in: Result<NewSignIn, ValidationErrors> = form.0.try_into();
    let new_sign_in = match new_sign_in {
        Ok(n) => n,
        Err(validation_errors) => {
            validation_errors
                .field_errors()
                .values()
                .flat_map(|&f| f.iter().map(|e| e.to_owned()))
                .map(|ve| ve.to_string())
                .for_each(|e| FlashMessage::error(e).send());
            return see_other("/signin");
        }
    };

    match get_current_user_details(&pool, new_sign_in.email).await {
        Ok(d) if d.is_some() => {
            let (user_id, expected_password_hash, role) = d.unwrap();

            let is_password_valid_result = spawn_blocking(move || {
                verify_password_hash(
                    Secret::new(expected_password_hash),
                    Secret::new(new_sign_in.password),
                )
            })
            .await
            .context("Failed to spawn blocking task.")
            .unwrap();

            match is_password_valid_result {
                Ok(()) => {
                    session.renew();
                    session
                        .insert_user_id(user_id)
                        .expect("Could not insert user_id into session!");
                    session
                        .insert_role(&role)
                        .expect("Could not insert role into session");
                    see_other("/")
                }
                Err(e) => match e {
                    AuthError::InvalidCredentials(e) => {
                        log::error!("Invalid auth {}", e);
                        FlashMessage::new("Invalid credentials".into(), Level::Error).send();
                        see_other("/signin")
                    }
                    AuthError::UnexpectedError(e) => {
                        log::error!("Error checking credentials {}", e);
                        FlashMessage::new("Invalid credentials".into(), Level::Error).send();
                        see_other("/signin")
                    }
                },
            }
        }
        Ok(_) => {
            log::error!("No such credentials!");
            FlashMessage::error("Invalid credentials").send();
            see_other("/signin")
        }
        Err(e) => {
            log::error!("Server error occurred: {}", e);
            FlashMessage::error("Ooops. It's not You, it's us! Try again later!").send();
            see_other("/signin")
        }
    }
}

async fn get_current_user_details(
    pool: &PgPool,
    email: String,
) -> Result<Option<(Uuid, String, String)>, AuthError> {
    let details = sqlx::query!(
        r#"
        SELECT id, password_hash, role FROM users where email = $1
        "#,
        email,
    )
    .fetch_optional(pool)
    .await
    .context("Failed to get user details")?
    .map(|row| (row.id, row.password_hash, row.role));
    Ok(details)
}
