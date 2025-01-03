use auth::models::user_model::User;
use axum::{debug_handler, extract::State, http::StatusCode, Extension, Json};
use mongodb::{results::InsertOneResult, Collection};

use crate::models::{board_model::{Board, BoardCreate}, board_response_model::BoardError, category::Category};

#[debug_handler]
pub async  fn create_board(
    State(mongo): State<Collection<Board>>,
    Extension(current_user): Extension<User>,
    Json(new_board): Json<BoardCreate>
)-> Result<Json<InsertOneResult>, BoardError>{

    let categories = vec![
        Category{ name: "To Do".to_string(), color: "#89CFF0".to_string() },
        Category{ name: "In Progress".to_string(), color: "#ADD8E6".to_string() },
        Category{ name: "Done".to_string(), color: "#00A36C".to_string() },
    ];

    let board = Board{
        id: None,
        name: new_board.name,
        description: new_board.description,
        created_by: current_user.email.clone(),
        members: vec![current_user.email.clone()],
        categories: categories,
        issues: vec![],
        
    };

    match mongo.insert_one(board).await{
        Ok(result) => return Ok(Json(result)),
        Err(err) => return Err(BoardError{
            message: format!("Error inserting a board : {err}"),
            status_code: StatusCode::INTERNAL_SERVER_ERROR,
        }),
    };

}