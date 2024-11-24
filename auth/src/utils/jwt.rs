use axum::http::StatusCode;
use chrono::{Duration, Utc};
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, TokenData, Validation};
use std::env;

use crate::models::jwt_model::Payload;

use super::response::internal_error;

pub fn encode_jwt(email: &String)->Result<String, (StatusCode, String)>{
    let secret = env::var("JWT_SECRET").map_err(internal_error)?;
    let now = Utc::now();
    let expire = Duration::minutes(5);
    let exp = (now + expire).timestamp() as usize;
    let iat = now.timestamp() as usize;
    let email = email.clone();
    let claim = Payload{exp, iat,email};

    encode(
        &Header::default(), 
        &claim,
        &EncodingKey::from_secret(secret.as_ref())
    )
    .map_err(internal_error)
}

pub fn encode_refresh_jwt(email: &String)->Result<String, (StatusCode, String)>{
    let secret = env::var("REFRESH_JWT_SECRET").map_err(internal_error)?;
    let now = Utc::now();
    let expire = Duration::days(30);
    let exp = (now + expire).timestamp() as usize;
    let iat = now.timestamp() as usize;
    let email = email.clone();
    let claim = Payload{exp, iat,email};

    encode(
        &Header::default(), 
        &claim,
        &EncodingKey::from_secret(secret.as_ref())
    )
    .map_err(internal_error)
}

pub fn decode_refresh_jwt(jwt_token: String) -> Result<TokenData<Payload>, (StatusCode, String)>{
    let secret = env::var("REFRESH_JWT_SECRET").map_err(internal_error)?;
    let result: TokenData<Payload> = decode(
        &jwt_token, 
        &DecodingKey::from_secret(secret.as_ref()),
        &Validation::default()
    )
    .map_err(internal_error)?;
    
    Ok(result)

}

pub fn decode_jwt(jwt_token: String) -> Result<TokenData<Payload>, (StatusCode, String)>{
    let secret = env::var("JWT_SECRET").map_err(internal_error)?;
    let result: TokenData<Payload> = decode(
        &jwt_token, 
        &DecodingKey::from_secret(secret.as_ref()),
        &Validation::default()
    )
    .map_err(internal_error)?;
    
    Ok(result)

}