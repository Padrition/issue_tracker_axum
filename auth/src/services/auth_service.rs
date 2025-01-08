use axum::{extract::State, http::StatusCode, Json};
use mongodb::Collection;

use crate::{
    models::{auth_model::SignInData, jwt_model::TokenResponse, user_model::User},
    repository::user_repository::retrieve_user_by_email,
    utils::{
        jwt::{decode_refresh_jwt, encode_jwt, encode_refresh_jwt},
        response::internal_error,
    },
};

use argon2::verify_encoded;

pub async fn sign_in(
    State(mongo): State<Collection<User>>,
    Json(user_data): Json<SignInData>,
) -> Result<Json<TokenResponse>, (StatusCode, String)> {
    let user = match retrieve_user_by_email(&mongo, &user_data.email).await {
        Ok(user) => match user {
            Some(user) => user,
            None => return Err((StatusCode::UNAUTHORIZED, "Unauthorized".to_string())),
        },
        Err(_) => {
            return Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                "Error retrieving user".to_string(),
            ))
        }
    };

    match verify_encoded(&user.password_hash, &user_data.password.as_bytes()) {
        Ok(result) => {
            if !result {
                return Err((StatusCode::UNAUTHORIZED, "Unauthorized".to_string()));
            } else {
                let access_token = match encode_jwt(&user.email) {
                    Ok(token) => token,
                    Err(err) => {
                        return Err(err);
                    }
                };

                let refresh_token = match encode_refresh_jwt(&user.email) {
                    Ok(token) => token,
                    Err(err) => {
                        return Err(err);
                    }
                };

                Ok(Json(TokenResponse {
                    access_token: access_token,
                    refresh_token: refresh_token,
                }))
            }
        }
        Err(err) => return Err(internal_error(err)),
    }
}

pub async fn refresh(
    State(mongo): State<Collection<User>>,
    Json(refresh_token): Json<String>,
) -> Result<Json<TokenResponse>, (StatusCode, String)> {
    let token_data = match decode_refresh_jwt(refresh_token) {
        Ok(data) => data,
        Err(err) => return Err(err),
    };

    let user = match retrieve_user_by_email(&mongo, &token_data.claims.email).await {
        Ok(user) => match user {
            Some(user) => user,
            None => return Err((StatusCode::UNAUTHORIZED, "Unauthorized".to_string())),
        },
        Err(_) => {
            return Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                "Error retrieving user".to_string(),
            ))
        }
    };

    let access_token = match encode_jwt(&user.email) {
        Ok(token) => token,
        Err(err) => {
            return Err(err);
        }
    };

    let refresh_token = match encode_refresh_jwt(&user.email) {
        Ok(token) => token,
        Err(err) => {
            return Err(err);
        }
    };

    Ok(Json(TokenResponse {
        access_token: access_token,
        refresh_token: refresh_token,
    }))
}
