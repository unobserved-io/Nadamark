use axum::{http::StatusCode, Json};

use crate::{
    database::{self, is_subfolder},
    models::{ItemType, MoveItemRequest},
};

pub async fn handle_move(Json(payload): Json<MoveItemRequest>) -> Result<StatusCode, StatusCode> {
    if payload.item_type == ItemType::Folder {
        // Prevent moving folder into itself or its children
        if payload.target_folder_id.is_some() {
            if is_subfolder(payload.item_id, payload.target_folder_id.unwrap_or(0)) {
                return Err(StatusCode::BAD_REQUEST);
            }
        }

        if let Err(e) = database::change_folder_parent(payload.item_id, payload.target_folder_id) {
            eprintln!("Failed to change folder's parent id: {}", e);
            return Err(StatusCode::INTERNAL_SERVER_ERROR);
        }
    } else if payload.item_type == ItemType::Bookmark {
        if let Err(e) = database::change_bookmark_folder(payload.item_id, payload.target_folder_id)
        {
            eprintln!("Failed to change bookmark's folder id: {}", e);
            return Err(StatusCode::INTERNAL_SERVER_ERROR);
        }
    } else {
        return Err(StatusCode::BAD_REQUEST);
    }

    Ok(StatusCode::OK)
}
