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
    results::{DeleteResult, InsertOneResult},
    Collection,
};

use crate::models::{
    board_model::{Board, BoardCreate, BoardResponse, BoardUpdate},
    board_response_model::BoardError,
    category::Category,
};

#[debug_handler]
pub async fn create_board(
    State(mongo): State<Collection<Board>>,
    Extension(current_user): Extension<User>,
    Json(new_board): Json<BoardCreate>,
) -> Result<Json<InsertOneResult>, BoardError> {
    let categories = vec![
        Category {
            name: "To Do".to_string(),
            color: "#89CFF0".to_string(),
        },
        Category {
            name: "In Progress".to_string(),
            color: "#ADD8E6".to_string(),
        },
        Category {
            name: "Done".to_string(),
            color: "#00A36C".to_string(),
        },
    ];

    let board = Board {
        id: None,
        name: new_board.name,
        description: new_board.description,
        created_by: current_user.email.clone(),
        members: vec![current_user.email.clone()],
        categories: categories,
        issues: vec![],
    };

    match mongo.insert_one(board).await {
        Ok(result) => return Ok(Json(result)),
        Err(err) => {
            return Err(BoardError {
                message: format!("Error inserting a board : {err}"),
                status_code: StatusCode::INTERNAL_SERVER_ERROR,
            })
        }
    };
}

#[debug_handler]
pub async fn update_board(
    State(mongo): State<Collection<Board>>,
    Extension(current_user): Extension<User>,
    Json(board_update): Json<BoardUpdate>,
) -> Result<Json<Board>, BoardError> {
    let existing_board = match mongo.find_one(doc! {"_id": &board_update.id }).await {
        Ok(result) => match result {
            Some(board) => {
                if &current_user.email != &board.created_by {
                    return Err(BoardError {
                        message: "Forbidden: Board must be updated by a creator".to_string(),
                        status_code: StatusCode::FORBIDDEN,
                    });
                }

                board
            }
            None => {
                return Err(BoardError {
                    message: "Board not found".to_string(),
                    status_code: StatusCode::NOT_FOUND,
                });
            }
        },
        Err(err) => {
            return Err(BoardError {
                message: format!("Error finding board {err}"),
                status_code: StatusCode::INTERNAL_SERVER_ERROR,
            })
        }
    };

    let updated_board = Board {
        id: Some(board_update.id),
        name: board_update.name.unwrap_or(existing_board.name),
        description: board_update
            .description
            .unwrap_or(existing_board.description),
        created_by: existing_board.created_by,
        members: board_update.members.unwrap_or(existing_board.members),
        categories: board_update.categories.unwrap_or(existing_board.categories),
        issues: existing_board.issues,
    };

    match mongo
        .replace_one(doc! {"_id": &board_update.id}, &updated_board)
        .await
    {
        Ok(_) => match mongo.find_one(doc! {"_id": &board_update.id}).await {
            Ok(result) => match result {
                Some(board) => return Ok(Json(board)),
                None => {
                    return Err(BoardError {
                        message: "Error reading board after update: Board not found".to_string(),
                        status_code: StatusCode::NOT_FOUND,
                    })
                }
            },
            Err(err) => Err(BoardError {
                message: format!("Error reading board after update: {err}"),
                status_code: StatusCode::INTERNAL_SERVER_ERROR,
            }),
        },
        Err(err) => Err(BoardError {
            message: format!("Error updating board: {err}"),
            status_code: StatusCode::INTERNAL_SERVER_ERROR,
        }),
    }
}

#[debug_handler]
pub async fn get_boards(
    State(mongo): State<Collection<Board>>,
    Extension(current_user): Extension<User>,
) -> Result<Json<Vec<Board>>, BoardError> {
    match mongo.find(doc! {"members": &current_user.email}).await {
        Ok(cursor) => {
            let boards: Vec<Board> = cursor.try_collect().await.map_err(|err| BoardError {
                message: format!("Error collecting boards: {err}"),
                status_code: StatusCode::INTERNAL_SERVER_ERROR,
            })?;
            Ok(Json(boards))
        }
        Err(err) => {
            return Err(BoardError {
                message: format!("Error finding boards: {err}"),
                status_code: StatusCode::INTERNAL_SERVER_ERROR,
            })
        }
    }
}

#[debug_handler]
pub async fn delete_board(
    State(mongo): State<Collection<Board>>,
    Extension(current_user): Extension<User>,
    Path(id): Path<String>,
) -> Result<Json<DeleteResult>, BoardError> {
    let objid = ObjectId::parse_str(&id).map_err(|err| BoardError {
        message: format!("Error parsing id to ObjectId: {err}"),
        status_code: StatusCode::INTERNAL_SERVER_ERROR,
    })?;

    match mongo.find_one(doc! {"_id": objid}).await {
        Ok(result) => match result {
            Some(board) => {
                if &current_user.email != &board.created_by {
                    return Err(BoardError {
                        message: "Forbidden: Board must be deleted by a creator".to_string(),
                        status_code: StatusCode::FORBIDDEN,
                    });
                }

                match mongo.delete_one(doc! {"_id": objid}).await {
                    Ok(result) => return Ok(Json(result)),
                    Err(err) => {
                        return Err(BoardError {
                            message: format!("Error deleting board: {err}"),
                            status_code: StatusCode::INTERNAL_SERVER_ERROR,
                        });
                    }
                }
            }
            None => {
                return Err(BoardError {
                    message: "Board not found".to_string(),
                    status_code: StatusCode::NOT_FOUND,
                })
            }
        },
        Err(err) => {
            return Err(BoardError {
                message: format!("Error finding board: {err}"),
                status_code: StatusCode::INTERNAL_SERVER_ERROR,
            });
        }
    }
}

pub async fn get_board(
    State(mongo): State<Collection<Board>>,
    Extension(current_user): Extension<User>,
    Path(id): Path<String>,
) -> Result<Json<BoardResponse>, BoardError> {
    let objid = ObjectId::parse_str(&id).map_err(|err| BoardError {
        message: format!("Error parsing id to ObjectId: {err}"),
        status_code: StatusCode::INTERNAL_SERVER_ERROR,
    })?;

    match mongo.find_one(doc! {"_id": objid}).await {
        Ok(result) => match result {
            Some(board) => {
                if !board.members.contains(&current_user.email) {
                    return Err(BoardError {
                        message: "Forbidden: You must be a board member".to_string(),
                        status_code: StatusCode::FORBIDDEN,
                    });
                }
                return Ok(Json(BoardResponse {
                    id,
                    name: board.name,
                    description: board.description,
                    is_creator: board.created_by.eq(&current_user.email),
                    members: board.members,
                    categories: board.categories,
                    issues: board.issues,
                    created_by: board.created_by,
                }));
            }
            None => {
                return Err(BoardError {
                    message: "Board not found".to_string(),
                    status_code: StatusCode::NOT_FOUND,
                })
            }
        },
        Err(err) => {
            return Err(BoardError {
                message: format!("Error finding board: {err}"),
                status_code: StatusCode::INTERNAL_SERVER_ERROR,
            })
        }
    }
}
