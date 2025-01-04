use axum::http::StatusCode;
use mongodb::{bson::doc, Collection};

use crate::{models::user_model::User, utils::response::internal_error};

pub async fn retrieve_user_by_email(
    mongo: &Collection<User>,
    email: &str,
) -> Result<Option<User>, (StatusCode, String)> {
    let user = mongo
        .find_one(doc! {"email": email})
        .await
        .map_err(internal_error)?;

    Ok(user)
}
