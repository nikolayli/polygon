use lettre::message::Message;
use lettre::transport::smtp::authentication::Credentials;
use lettre::transport::smtp::SmtpTransport;
use lettre::AsyncSmtpTransport;
use lettre::Tokio1Executor;
use std::env;

pub async fn send_verification_email(to: &str, token: &str) {
    let smtp_server = env::var("SMTP_SERVER").expect("SMTP_SERVER must be set");
    let smtp_username = env::var("SMTP_USERNAME").expect("SMTP_USERNAME must be set");
    let smtp_password = env::var("SMTP_PASSWORD").expect("SMTP_PASSWORD must be set");

    let email = Message::builder()
        .from(smtp_username.parse().unwrap())
        .to(to.parse().unwrap())
        .subject("Verify your email")
        .body(format!("Please verify your email using this token: {}", token))
        .unwrap();

    let creds = Credentials::new(smtp_username, smtp_password);

    let mailer: AsyncSmtpTransport<Tokio1Executor> = SmtpTransport::relay(&smtp_server)
        .unwrap()
        .credentials(creds)
        .build();

    mailer.send(email).await.unwrap();    
}