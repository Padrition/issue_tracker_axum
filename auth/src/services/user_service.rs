use crate::models::user_model::CurrentUser;
use argon2::{Config, hash_encoded};

pub fn retrieve_user_by_email(email: &str)-> Option<CurrentUser>{
    let password = b"password";
    let salt = &rand::random::<[u8;16]>();
    let config = Config::default();

    let hash = hash_encoded(password, salt, &config).expect("Password hashing failed.");

    let current_user: CurrentUser = CurrentUser { 
        email: "padrition@gmail.com".to_string(), 
        login: "Padrition".to_string(),
        password_hash: hash,
    };

    Some(current_user)
}