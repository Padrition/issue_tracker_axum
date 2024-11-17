use axum::{http::StatusCode, Json};

use crate::{models::auth_model::SignInData, utils::{jwt::encode_jwt, response::internal_error}};

use argon2::verify_encoded;

use super::user_service::retrieve_user_by_email;

pub async fn sign_in(
    Json(user_data) : Json<SignInData>
) -> Result<Json<String>, (StatusCode, String)>{

    let user = match retrieve_user_by_email(&user_data.email){
        Some(user) => user,
        None => return Err((StatusCode::UNAUTHORIZED, "Unauthorized".to_string())),
    };

    match verify_encoded(&user.password_hash, &user_data.password.as_bytes()){
        Ok(result) => {
            if !result{
                return Err((StatusCode::UNAUTHORIZED, "Unauthorized".to_string()));
            }else{
                let token = encode_jwt(user.email);
                match token{
                    Ok(token) => {
                        return Ok(Json(token));
                    },
                    Err(err)=>{
                        return Err(err);
                    },
                }
            }
        },
        Err(err) => {
            return Err(internal_error(err))
        },
    }
        
    
}

