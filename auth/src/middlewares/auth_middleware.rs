use axum::{body::Body, extract::{Request, State}, http::{self, Response, StatusCode}, middleware::Next};
use mongodb::Collection;

use crate::{models::{auth_model::AuthError, user_model::User}, repository::user_repository::retrieve_user_by_email, utils::jwt::decode_jwt};

pub async fn authorization_middleware(
    State(mongo): State<Collection<User>>,
    mut req: Request,
    next: Next
) -> Result<Response<Body>, AuthError>{
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
        Err(err) => return Err(AuthError { 
            message: format!("Unable to decode token :{}", err.1),
            status_code: StatusCode::UNAUTHORIZED 
            }),
    };

    let current_user = match retrieve_user_by_email(&mongo,&token_data.claims.email).await{
        Ok(user) => match user {
            Some(user) => user,
            None => return Err(AuthError{
                message: "You are not an authorized user".to_string(),
                status_code: StatusCode::UNAUTHORIZED,
            }),
        },
        Err(_) => return Err(AuthError { 
            message: "Error finding user".to_string(),
            status_code: StatusCode::INTERNAL_SERVER_ERROR
        }),
    };
    req.extensions_mut().insert(current_user);
    Ok(next.run(req).await)

}