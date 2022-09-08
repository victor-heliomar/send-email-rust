use dotenv;
use std::env;
use lettre::Transport;

mod sender;
mod mail;
use sender::Sender;
use mail::mail;

fn main() {
    dotenv::dotenv().ok();
    let sender: Sender = Sender::new(env::var("EMAIL").unwrap(), env::var("PASSWORD").unwrap());
    
    let mut args: env::Args = env::args();

    let recipient = args.nth(1).expect("Recipient email is not found");
    let subject = args.nth(0).expect("Subject is not found");
    let body = args.nth(0).expect("Body is not found").to_string();

    let (email, mailer) = mail(sender, recipient, subject, body);

    // Send the email
    match mailer.send(&email) {
        Ok(_) => println!("Email sent successfully!"),
        Err(e) => panic!("Could not send email: {:?}", e),
    }
}