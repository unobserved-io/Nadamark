use axum::{http::StatusCode, response::IntoResponse, Json};

use crate::{database, models::NewFolderRequest};

pub async fn create_folder(Json(payload): Json<NewFolderRequest>) -> impl IntoResponse {
    if let Err(e) = database::create_new_folder(payload.name, payload.parent_id) {
        eprintln!("Failed to create folder: {}", e);
        return Err(StatusCode::INTERNAL_SERVER_ERROR);
    }

    Ok(StatusCode::OK)
}
