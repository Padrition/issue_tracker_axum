use std::{fmt::format, result};

use auth::models::user_model::User;
use axum::{extract::State, http::StatusCode, Extension, Json};
use mongodb::{
    bson::{doc, oid::ObjectId},
    results::InsertOneResult,
};

use crate::{
    models::{
        issue_model::{Issue, IssueCreate, Priority},
        issue_response::IssueError,
    },
    utils::app_state::AppState,
};

pub async fn create_issue(
    State(state): State<AppState>,
    Extension(current_user): Extension<User>,
    Json(new_issue): Json<IssueCreate>,
) -> Result<Json<InsertOneResult>, IssueError> {
    let board_collection = &state.board_collection;
    let issue_collection = &state.issue_collection;

    let board_objid = ObjectId::parse_str(&new_issue.board_id).map_err(|err| IssueError {
        message: format!("Error parsing id to ObjectId: {err}"),
        status_code: StatusCode::INTERNAL_SERVER_ERROR,
    })?;

    let board = match board_collection.find_one(doc! {"_id" : board_objid}).await {
        Ok(result) => match result {
            Some(board) => board,
            None => {
                return Err(IssueError {
                    message: "Board not found".to_string(),
                    status_code: StatusCode::NOT_FOUND,
                })
            }
        },
        Err(err) => {
            return Err(IssueError {
                message: format!("Error finding board : {err}"),
                status_code: StatusCode::INTERNAL_SERVER_ERROR,
            })
        }
    };

    if !board.members.contains(&current_user.email) {
        return Err(IssueError {
            message: "Forbidden".to_string(),
            status_code: StatusCode::FORBIDDEN,
        });
    }

    let issue = Issue {
        id: None,
        title: new_issue.title,
        description: new_issue.description,
        status: "To Do".to_string(),
        priority: Priority::Medium,
    };

    let insert_result = issue_collection
        .insert_one(issue)
        .await
        .map_err(|err| IssueError {
            message: format!("Error inserting issue: {err}"),
            status_code: StatusCode::INTERNAL_SERVER_ERROR,
        })?;

    let issue_id = insert_result
        .inserted_id
        .as_object_id()
        .ok_or_else(|| IssueError {
            message: "Failed to retrieve issue is".to_string(),
            status_code: StatusCode::INTERNAL_SERVER_ERROR,
        })?;

    board_collection
        .update_one(
            doc! {"_id": board_objid},
            doc! {"$push": {"issues": issue_id}},
        )
        .await
        .map_err(|err| IssueError {
            message: format!("Error updating board: {err}"),
            status_code: StatusCode::INTERNAL_SERVER_ERROR,
        })?;

    Ok(Json(insert_result))
}
