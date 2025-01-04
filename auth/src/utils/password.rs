use argon2::{hash_encoded, Config};

pub async fn hash_password(password: &String) -> Option<String> {
    let salt = &rand::random::<[u8; 16]>();
    let config = Config::default();

    let hashed_password =
        hash_encoded(password.as_bytes(), salt, &config).expect("Error hashing password.");

    Some(hashed_password)
}
