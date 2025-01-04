pub mod middlewares;
pub mod models;
pub mod repository;
pub mod services;
pub mod utils;

pub use middlewares::auth_middleware;
pub use models::auth_model::{AuthError, SignInData};
pub use models::jwt_model::Payload;
pub use services::auth_service::sign_in;
pub use utils::{
    db::connect_to_mongo,
    jwt::{decode_jwt, encode_jwt},
    shutdown::shutdown_signal,
};
