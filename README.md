# SEND-EMAIL-RUST

SEND-EMAIL-RUST is a program to send emails through the command line console.

### How to use

1. First you needs to configure your e-mail to send mails through SMTP.
2. Clone the repo.
3. Build the APP `cargo build --release`.
4. Add a .env file with your credentials.
5. Execute the bin file created `target/release/send-email-rust [recipient-email] [subject] [body]`.
6. You'll receive a message in console with the request status.