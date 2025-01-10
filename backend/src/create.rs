use axum::{http::StatusCode, response::IntoResponse, Json};
use serde::Deserialize;

use crate::database;

#[derive(Debug, Deserialize)]
pub struct NewFolderRequest {
    name: String,
    parent_id: Option<i32>,
}

#[derive(Debug, Deserialize)]
pub struct NewBookmarkRequest {
    name: String,
    url: String,
    folder_id: Option<i32>,
}

pub async fn create_folder(Json(payload): Json<NewFolderRequest>) -> impl IntoResponse {
    if let Err(e) = database::create_new_folder(payload.name, payload.parent_id) {
        eprintln!("Failed to create folder: {}", e);
        return Err(StatusCode::INTERNAL_SERVER_ERROR);
    }

    Ok(StatusCode::OK)
}

pub async fn create_bookmark(Json(payload): Json<NewBookmarkRequest>) -> impl IntoResponse {
    if let Err(e) = database::create_new_bookmark(payload.name, payload.url, payload.folder_id) {
        eprintln!("Failed to create bookmark: {}", e);
        return Err(StatusCode::INTERNAL_SERVER_ERROR);
    }

    Ok(StatusCode::OK)
}
