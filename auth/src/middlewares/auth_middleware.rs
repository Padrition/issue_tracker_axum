use axum::{body::Body, extract::Request, http::{self, Response, StatusCode}, middleware::Next};

use crate::{models::auth_model::AuthError, services::user_service::retrieve_user_by_email, utils::jwt::decode_jwt};

pub async fn authorization_middleware(mut req: Request, next: Next) -> Result<Response<Body>, AuthError>{
    let auth_header = req.headers_mut().get(http::header::AUTHORIZATION);
    let auth_header = match auth_header{
        Some(header) => {
            header.to_str().map_err(|_| AuthError{
                message: "Empty header is not allowed".to_string(),
                status_code: StatusCode::FORBIDDEN
            })
        }?,
        None => {
            return Err(AuthError{
                message: "Please add the JWT token to the header".to_string(),
                status_code: StatusCode::FORBIDDEN,
            })
        },
    };

    let mut header = auth_header.split_whitespace();
    let (_bearer, token) = (header.next(), header.next());
    let token_data = match decode_jwt(token.unwrap().to_string()){
        Ok(data) => data,
        Err(_) => return Err(AuthError { 
            message: "Unable to decode token".to_string(),
             status_code: StatusCode::UNAUTHORIZED 
            }),
    };

    let current_user = match retrieve_user_by_email(&token_data.claims.email){
        Some(user) => user,
        None => return Err(AuthError{
            message: "You are not an authorized user".to_string(),
            status_code: StatusCode::UNAUTHORIZED,
        }),
    };
    req.extensions_mut().insert(current_user);
    Ok(next.run(req).await)

}