use lettre::transport::smtp::authentication::Credentials;
use lettre::{Message, SmtpTransport};
use lettre::message::{MultiPart, SinglePart, header};

use crate::sender::Sender;

pub fn mail(sender: Sender, recipient: String, subject: String, body: String) -> (Message, SmtpTransport) {
    let email = Message::builder()
        .from(sender.get_email().parse().unwrap())
        .to(recipient.parse().unwrap())
        .subject(subject)
        .multipart(
            MultiPart::alternative() // This is composed of two parts.
                .singlepart(
                    SinglePart::builder()
                        .header(header::ContentType::TEXT_PLAIN)
                        .body(String::from("Hello from Lettre! A mailer library for Rust")), // Every message should have a plain text fallback.
                )
                .singlepart(
                    SinglePart::builder()
                        .header(header::ContentType::TEXT_HTML)
                        .body(body),
                ),
        )
        .expect("Something was wrong while the email was created");

    let creds = Credentials::new(sender.get_email().to_string(), sender.get_secret().to_string());

    let mailer = SmtpTransport::relay("smtp.gmail.com")
        .unwrap()
        .credentials(creds)
        .build();

    (email, mailer)
}