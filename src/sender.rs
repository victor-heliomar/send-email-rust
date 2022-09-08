pub struct Sender {
    email: String,
    secret: String
}

impl Sender {
    pub fn new(email: String, password: String) -> Sender {
        Sender {
            email,
            secret: password,
        }
    }

    pub fn get_email(&self) -> &str {
        &self.email
    }
    
    pub fn get_secret(&self) -> &str {
        &self.secret
    }
}
