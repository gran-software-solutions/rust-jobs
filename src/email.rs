use anyhow::Context;
use log::info;
use rust_gmail::GmailClient;

pub async fn send_user_confirmation_email(
    recipient_email: &str,
    send_from_email: &str,
    service_account_file_full_path: &str,
) -> Result<(), anyhow::Error> {
    info!("Sending confirmation mail");
    let email_client = GmailClient::builder(service_account_file_full_path, send_from_email)?
        .build()
        .await
        .context("Could not build email_client")?;

    email_client
        .send_email(
            recipient_email,
            "Complete registration",
            "Please complete your registration",
        )
        .await
        .context("Could not send email")?;
    Ok(())
}
