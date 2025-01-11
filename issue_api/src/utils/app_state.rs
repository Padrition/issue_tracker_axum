use board_api::Board;
use mongodb::Collection;

use crate::models::issue_model::Issue;

#[derive(Clone)]
pub struct AppState {
    pub board_collection: Collection<Board>,
    pub issue_collection: Collection<Issue>,
}
