use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct PasswordEntry {
    service: String,
    username: Option<String>,
    password: String,
}

impl PasswordEntry {
    pub fn new(service: String, username: Option<String>, password: String) -> PasswordEntry {
        PasswordEntry {
            service,
            username,
            password,
        }
    }
    pub fn print_password(&self) {
        println!("Password: {}", self.password);
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Passwords(Vec<PasswordEntry>);

impl Passwords {
    pub fn append(&mut self, new_password: PasswordEntry) {
        self.0.push(new_password);
    }
    pub fn find(&self, service: &str, username: Option<String>) -> Option<&PasswordEntry> {
        self.0
            .iter()
            .find(|pwd| pwd.service == service && pwd.username == username)
    }
    pub fn remove(&mut self, service: &str, username: Option<String>) -> bool {
        {
            let initial_len = self.0.len();
            self.0
                .retain(|pwd| !(pwd.service == service && pwd.username == username));
            initial_len != self.0.len()
        }
    }
    pub fn parse_passwords(raw_passwords: &str) -> Result<Passwords, anyhow::Error> {
        let passwords: Passwords = serde_json::from_str(raw_passwords)?;
        Ok(passwords)
    }
    pub fn print_all(&self) {
        self.0.iter().for_each(|pwd| {
            if let Some(username) = &pwd.username {
                println!(
                    "Service: {}, Username: {}, Password: {}",
                    pwd.service, username, pwd.password
                )
            } else {
                println!("Service: {}, Password: {}", pwd.service, pwd.password)
            }
        })
    }
}