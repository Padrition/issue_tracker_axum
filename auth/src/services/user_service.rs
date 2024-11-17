use crate::{models::user_model::{CurrentUser, User, UserCreate, UserInsert}, utils::{password::hash_password, response::internal_error}};
use argon2::{Config, hash_encoded};
use axum::{extract::State, http::StatusCode, Json};
use mongodb::{results::InsertOneResult, Collection};

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

pub async fn create_user(
    State(mongo): State<Collection<UserInsert>>,
    Json(new_user): Json<UserCreate>, 
)-> Result<Json<InsertOneResult>, (StatusCode, String)>{

    let hashed_password = hash_password(&new_user.password)
        .await
        .expect("Error hashing the password");

    let user = UserInsert{
        email: new_user.email,
        login: new_user.login,
        password_hash: hashed_password,
    };

    let result  = mongo.insert_one(user)
        .await
        .map_err(internal_error)?;

    Ok(Json(result))
}