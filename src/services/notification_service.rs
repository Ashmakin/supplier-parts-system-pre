use lettre::{
    transport::smtp::{
        authentication::Credentials,
        client::{Tls, TlsParameters}, // <-- Import Tls and TlsParameters
    },
    Message, SmtpTransport, Transport,
};
use std::env;
use crate::errors::AppError;

pub async fn send_email(to: String, subject: String, body: String) -> Result<(), AppError> {
    let from = env::var("SMTP_FROM").expect("SMTP_FROM must be set");
    let smtp_user = env::var("SMTP_USER").expect("SMTP_USER must be set");
    let smtp_pass = env::var("SMTP_PASS").expect("SMTP_PASS must be set");
    let smtp_host = env::var("SMTP_HOST").expect("SMTP_HOST must be set");
    let smtp_port_str = env::var("SMTP_PORT").expect("SMTP_PORT must be set");
    let smtp_port = smtp_port_str.parse::<u16>().expect("SMTP_PORT must be a valid number");


    let email = Message::builder()
        .from(from.parse().unwrap())
        .to(to.parse().map_err(|_| AppError::InternalServerError("Invalid 'to' email address".to_string()))?)
        .subject(subject)
        .body(body)
        .map_err(|_| AppError::InternalServerError("Failed to build email".to_string()))?;

    let creds = Credentials::new(smtp_user, smtp_pass);

    // --- This is the corrected part ---
    // We are now building the transport client more explicitly to handle TLS correctly.

    // 1. Set up TLS parameters for the domain.
    let tls_parameters = TlsParameters::new(smtp_host.clone())
        .map_err(|_| AppError::InternalServerError("Failed to create TLS parameters".to_string()))?;

    // 2. Build the mailer transport
    let mailer = SmtpTransport::builder_dangerous(&smtp_host)
        .port(smtp_port)
        .credentials(creds)
        // Use Opportunistic TLS (STARTTLS), which is what Mailtrap expects on port 2525
        .tls(Tls::Opportunistic(tls_parameters))
        .build();

    // The rest of the function remains the same.
    tokio::spawn(async move {
        match mailer.send(&email) {
            Ok(_) => log::info!("Email sent successfully!"),
            Err(e) => log::error!("Could not send email: {:?}", e),
        }
    });

    Ok(())
}