use auth::models::user_model::User;
use axum::{
    debug_handler,
    extract::{Path, State},
    http::StatusCode,
    Extension, Json,
};
use futures::TryStreamExt;
use mongodb::{
    bson::{doc, oid::ObjectId},
    results::InsertOneResult,
};

use crate::{
    models::{
        issue_model::{Issue, IssueCreate, IssueUpdate, Priority},
        issue_response::IssueError,
    },
    utils::app_state::AppState,
};

#[debug_handler]
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
        status: new_issue.status.unwrap_or("To Do".to_string()),
        priority: new_issue.priority.unwrap_or(Priority::Medium),
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

#[debug_handler]
pub async fn get_issues(
    State(state): State<AppState>,
    Extension(current_user): Extension<User>,
    Path(id): Path<String>,
) -> Result<Json<Vec<Issue>>, IssueError> {
    let board_collection = &state.board_collection;
    let issue_collection = &state.issue_collection;

    let board_objid = ObjectId::parse_str(id).map_err(|err| IssueError {
        message: format!("Error parsing id to ObjectId: {err}"),
        status_code: StatusCode::INTERNAL_SERVER_ERROR,
    })?;

    let board = board_collection
        .find_one(doc! {"_id": board_objid})
        .await
        .map_err(|err| IssueError {
            message: format!("Error finding board: {err}"),
            status_code: StatusCode::INTERNAL_SERVER_ERROR,
        })?
        .ok_or(IssueError {
            message: "Board not found".to_string(),
            status_code: StatusCode::NOT_FOUND,
        })?;

    if !board.members.contains(&current_user.email) {
        return Err(IssueError {
            message: "Forbidden".to_string(),
            status_code: StatusCode::FORBIDDEN,
        });
    }

    let issues: Vec<Issue> = issue_collection
        .find(doc! {"_id" : {"$in" : board.issues}})
        .await
        .map_err(|err| IssueError {
            message: format!("Error finding issues : {err}"),
            status_code: StatusCode::INTERNAL_SERVER_ERROR,
        })?
        .try_collect()
        .await
        .map_err(|err| IssueError {
            message: format!("Error collection issues: {err}"),
            status_code: StatusCode::INTERNAL_SERVER_ERROR,
        })?;

    Ok(Json(issues))
}

#[debug_handler]
pub async fn get_issue(
    State(state): State<AppState>,
    Extension(current_user): Extension<User>,
    Path(id): Path<String>,
) -> Result<Json<Issue>, IssueError> {
    let board_collection = &state.board_collection;
    let issue_collection = &state.issue_collection;

    let issue_objid = ObjectId::parse_str(id).map_err(|err| IssueError {
        message: format!("Error parsing id to ObjectId: {err}"),
        status_code: StatusCode::INTERNAL_SERVER_ERROR,
    })?;

    let board = board_collection
        .find_one(doc! {"issues": issue_objid})
        .await
        .map_err(|err| IssueError {
            message: format!("Error finding board: {err}"),
            status_code: StatusCode::INTERNAL_SERVER_ERROR,
        })?
        .ok_or(IssueError {
            message: "Board not found".to_string(),
            status_code: StatusCode::NOT_FOUND,
        })?;

    if !board.members.contains(&current_user.email) {
        return Err(IssueError {
            message: "Forbidden".to_string(),
            status_code: StatusCode::FORBIDDEN,
        });
    }

    let issue = issue_collection
        .find_one(doc! {"_id": issue_objid})
        .await
        .map_err(|err| IssueError {
            message: format!("Error finding issue : {err}"),
            status_code: StatusCode::INTERNAL_SERVER_ERROR,
        })?
        .ok_or(IssueError {
            message: "Issue not found".to_string(),
            status_code: StatusCode::NOT_FOUND,
        })?;

    Ok(Json(issue))
}

#[debug_handler]
pub async fn update_issue(
    State(state): State<AppState>,
    Extension(current_user): Extension<User>,
    Json(issue_update): Json<IssueUpdate>,
) -> Result<Json<Issue>, IssueError> {
    let issue_collection = &state.issue_collection;
    let board_collection = &state.board_collection;

    let issue_objid = ObjectId::parse_str(&issue_update.id).map_err(|err| IssueError {
        message: format!("Error parsing id to ObjectId: {err}"),
        status_code: StatusCode::INTERNAL_SERVER_ERROR,
    })?;

    let board = board_collection
        .find_one(doc! {"issues": issue_objid})
        .await
        .map_err(|err| IssueError {
            message: format!("Error finding board: {err}"),
            status_code: StatusCode::INTERNAL_SERVER_ERROR,
        })?
        .ok_or(IssueError {
            message: "Board not found".to_string(),
            status_code: StatusCode::NOT_FOUND,
        })?;

    if !board.members.contains(&current_user.email) {
        return Err(IssueError {
            message: "Forbidden".to_string(),
            status_code: StatusCode::FORBIDDEN,
        });
    }

    let issue = issue_collection
        .find_one(doc! {"_id": issue_objid})
        .await
        .map_err(|err| IssueError {
            message: format!("Error finding issue: {err}"),
            status_code: StatusCode::INTERNAL_SERVER_ERROR,
        })?
        .ok_or({
            IssueError {
                message: "Issue ot found".to_string(),
                status_code: StatusCode::NOT_FOUND,
            }
        })?;

    let updated_issue = Issue {
        id: Some(issue_objid),
        title: issue_update.title.unwrap_or(issue.title),
        description: issue_update.description.unwrap_or(issue.description),
        status: issue_update.status.unwrap_or(issue.status),
        priority: issue_update.priority.unwrap_or(issue.priority),
    };

    issue_collection
        .replace_one(doc! {"_id": issue_objid}, updated_issue)
        .await
        .map_err(|err| IssueError {
            message: format!("Error replacing issue :{err}"),
            status_code: StatusCode::INTERNAL_SERVER_ERROR,
        })?;

    let new_issue = issue_collection
        .find_one(doc! {"_id": issue_objid})
        .await
        .map_err(|err| IssueError {
            message: format!("Error finding issue: {err}"),
            status_code: StatusCode::INTERNAL_SERVER_ERROR,
        })?
        .ok_or(IssueError {
            message: "Issue not found".to_string(),
            status_code: StatusCode::NOT_FOUND,
        })?;

    Ok(Json(new_issue))
}
