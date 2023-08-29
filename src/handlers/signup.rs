use core::result::Result::{Err, Ok};

use actix_web::{
    http::{header::LOCATION, StatusCode},
    rt::task::spawn_blocking,
    web, HttpResponse, ResponseError,
};
use actix_web_flash_messages::{FlashMessage, IncomingFlashMessages};
use anyhow::Context;
use log::error;
use maud::{html, Markup};
use rand::{distributions::Alphanumeric, thread_rng, Rng};
use serde::Deserialize;
use sqlx::{PgPool, Postgres, Transaction};

use uuid::Uuid;
use validator::{Validate, ValidationError, ValidationErrors};

use crate::{
    authentication,
    configuration::Gmail,
    domain::Role,
    email::{self},
    handlers::{footer, head, header},
    utils::{error_chain_fmt, see_other},
};

#[derive(Debug)]
struct SignupFormError {
    err: anyhow::Error,
}

impl std::fmt::Display for SignupFormError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Signup failed")
    }
}

impl ResponseError for SignupFormError {}

impl From<anyhow::Error> for SignupFormError {
    fn from(value: anyhow::Error) -> Self {
        Self { err: value }
    }
}

pub async fn signup_form(messages: IncomingFlashMessages) -> Markup {
    let msgs: Vec<_> = messages.iter().map(|f| f.content()).collect();
    html! {
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
    }
}

#[derive(serde::Deserialize, Debug)]
pub struct Signup {
    role: String,
    email: String,
    password: String,
    repeat_password: String,
}

#[derive(thiserror::Error)]
pub enum SignupError {
    #[error("{0:?}")]
    ValidationError(Vec<String>),
    #[error(transparent)]
    UnexpectedError(#[from] anyhow::Error),
}

impl From<ValidationErrors> for SignupError {
    fn from(value: ValidationErrors) -> Self {
        Self::ValidationError(
            value
                .field_errors()
                .values()
                .flat_map(|&f| f.iter().map(|e| e.to_owned()))
                .map(|ve| ve.to_string())
                .collect(),
        )
    }
}

impl std::fmt::Debug for SignupError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        error_chain_fmt(self, f)
    }
}

impl ResponseError for SignupError {
    fn status_code(&self) -> StatusCode {
        match self {
            SignupError::ValidationError(_) => StatusCode::BAD_REQUEST,
            Self::UnexpectedError(_) => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }

    fn error_response(&self) -> HttpResponse<actix_web::body::BoxBody> {
        match self {
            Self::UnexpectedError(_error) => HttpResponse::SeeOther()
                .insert_header((LOCATION, "/signups"))
                .finish(),
            Self::ValidationError(errors) => {
                for err in errors {
                    FlashMessage::error(err).send()
                }
                HttpResponse::SeeOther()
                    .insert_header((LOCATION, "/signups"))
                    .finish()
            }
        }
    }
}

#[derive(Debug, Validate, Deserialize)]
struct NewSignup {
    #[validate(custom = "validate_role")]
    role: String,
    #[validate(email(message = "Invalid email"))]
    email: String,
    #[validate(length(min = 8, message = "Password must be at least 8 chars long"))]
    password: String,
    #[validate(must_match(other = "password", message = "Passwrod must match repeated password"))]
    repeat_password: String,
}

fn validate_role(role: &str) -> Result<(), ValidationError> {
    let role: Result<Role, String> = role.to_owned().try_into();
    match role {
        Ok(_) => Ok(()),
        Err(_) => Err(ValidationError::new("Invalid role")),
    }
}

pub async fn signup(
    signup: web::Form<Signup>,
    pool: web::Data<PgPool>,
    gmail: web::Data<Gmail>,
) -> Result<HttpResponse, SignupError> {
    let new_signup = NewSignup {
        email: signup.email.clone(),
        password: signup.password.clone(),
        repeat_password: signup.repeat_password.clone(),
        role: signup.role.clone(),
    };
    new_signup.validate()?;

    let mut transaction = pool
        .begin()
        .await
        .context("Failed to acquire a Postgres connection from the pool")?;

    let user_id = save_user(&mut transaction, new_signup)
        .await
        .context("Could not save user")?;

    let confirmation_token = generate_confirmation_token();

    store_token(&mut transaction, user_id, &confirmation_token)
        .await
        .context("Unable to save confirmation token")?;

    transaction
        .commit()
        .await
        .context("Could not commit transaction")?;

    if let Err(e) = email::send_user_confirmation_email(
        signup.email.as_str(),
        &gmail.send_from_email,
        &gmail.service_account_file_full_path,
    )
    .await
    {
        log::error!("Error sending registration confirmation mail: {}", e);
    };

    Ok(see_other("/"))
}

fn generate_confirmation_token() -> String {
    let mut rng: rand::rngs::ThreadRng = thread_rng();
    std::iter::repeat_with(|| rng.sample(Alphanumeric))
        .map(char::from)
        .take(25)
        .collect()
}

async fn store_token(
    transaction: &mut Transaction<'static, Postgres>,
    user_id: Uuid,
    token: &String,
) -> Result<(), anyhow::Error> {
    sqlx::query!(
        r#"
        INSERT INTO confirmation_tokens (confirmation_token, user_id) VALUES ($1, $2)
        "#,
        token,
        user_id
    )
    .execute(&mut **transaction)
    .await?;
    Ok(())
}

async fn save_user(
    transaction: &mut Transaction<'static, Postgres>,
    new_signup: NewSignup,
) -> Result<Uuid, anyhow::Error> {
    let user_id = uuid::Uuid::new_v4();
    let password = new_signup.password.clone();
    let hash = spawn_blocking(move || authentication::compute_password_hash(password))
        .await
        .context("Could not compute hash")??;
    let email = new_signup.email.clone();
    sqlx::query!(
        r#"
        INSERT INTO users(id, role, email, password_hash) VALUES ($1, $2, $3, $4)
        "#,
        user_id,
        new_signup.role,
        email,
        hash,
    )
    .execute(&mut **transaction)
    .await?;
    Ok(user_id)
}
