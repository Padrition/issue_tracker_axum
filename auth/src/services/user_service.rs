use crate::{models::user_model::{ User, UserCreate}, utils::{password::hash_password, response::internal_error}};
use axum::{extract::State, http::StatusCode, Json};
use mongodb::{ results::InsertOneResult, Collection};

pub async fn create_user(
    State(mongo): State<Collection<User>>,
    Json(new_user): Json<UserCreate>, 
)-> Result<Json<InsertOneResult>, (StatusCode, String)>{

    let hashed_password = hash_password(&new_user.password)
        .await
        .expect("Error hashing the password");

    let user = User{
        id: None,
        email: new_user.email,
        login: new_user.login,
        password_hash: hashed_password,
    };

    let result  = mongo.insert_one(user)
        .await
        .map_err(internal_error)?;

    Ok(Json(result))
}