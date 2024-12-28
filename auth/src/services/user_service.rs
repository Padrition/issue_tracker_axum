use crate::{
    models::{
        user_model::{
            User,
            UserCreate
        },
        auth_model::AuthError,
    },
    repository::user_repository::retrieve_user_by_email,
    utils::password::hash_password
};
use axum::{extract::State, http::StatusCode, Json};
use mongodb::{ results::InsertOneResult, Collection};

pub async fn create_user(
    State(mongo): State<Collection<User>>,
    Json(new_user): Json<UserCreate>, 
)-> Result<Json<InsertOneResult>, AuthError>{

    match retrieve_user_by_email(&mongo,&new_user.email).await{
        Ok(user) => match user {
            Some(_) => {
                return Err(AuthError{
                    status_code: StatusCode::CONFLICT,
                    message: "Email already in use".to_string()});
            },
            None => {
                let hashed_password = hash_password(&new_user.password)
                .await
                .expect("Error hashing the password");
        
                let user = User{
                    id: None,
                    email: new_user.email,
                    login: new_user.login,
                    password_hash: hashed_password,
                };
            
                let result  = match mongo.insert_one(user).await{
                    Ok(result) => result,
                    Err(_) => return Err(AuthError { 
                        message: "Error finding user".to_string(),
                        status_code: StatusCode::INTERNAL_SERVER_ERROR
                    }),
                };
            
                Ok(Json(result))
            },
        },
        Err(_) => return Err(AuthError { 
            message: "Error finding user".to_string(),
            status_code: StatusCode::INTERNAL_SERVER_ERROR
        }),
    }
}