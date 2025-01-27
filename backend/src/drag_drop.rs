use std::sync::Arc;

use axum::{extract::State, http::StatusCode, Json};

use crate::{
    database::{self, is_subfolder, Pool},
    models::{ItemType, MoveItemRequest},
};

pub async fn handle_move(
    State(pool): State<Arc<Pool>>,
    Json(payload): Json<MoveItemRequest>,
) -> Result<StatusCode, StatusCode> {
    let mut connection = pool.get().expect("Failed to get connection from pool");
    if payload.item_type == ItemType::Folder {
        // Prevent moving folder into itself or its children
        if payload.target_folder_id.is_some() {
            if is_subfolder(
                &mut connection,
                payload.item_id,
                payload.target_folder_id.unwrap_or(0),
            ) {
                return Err(StatusCode::BAD_REQUEST);
            }
        }

        if let Err(e) = database::change_folder_parent(
            &mut connection,
            payload.item_id,
            payload.target_folder_id,
        ) {
            eprintln!("Failed to change folder's parent id: {}", e);
            return Err(StatusCode::INTERNAL_SERVER_ERROR);
        }
    } else if payload.item_type == ItemType::Bookmark {
        if let Err(e) = database::change_bookmark_folder(
            &mut connection,
            payload.item_id,
            payload.target_folder_id,
        ) {
            eprintln!("Failed to change bookmark's folder id: {}", e);
            return Err(StatusCode::INTERNAL_SERVER_ERROR);
        }
    } else {
        return Err(StatusCode::BAD_REQUEST);
    }

    Ok(StatusCode::OK)
}
