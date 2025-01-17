use axum::{http::StatusCode, response::IntoResponse, Json};

use crate::{
    database,
    models::{UpdateBookmarkRequest, UpdateFolderRequest},
};

pub async fn favorite_bookmark(Json(payload): Json<i32>) -> impl IntoResponse {
    if let Err(e) = database::toggle_bookmark_favorite(payload) {
        eprintln!("Failed to toggle favorite: {}", e);
        return Err(StatusCode::INTERNAL_SERVER_ERROR);
    }

    Ok(StatusCode::OK)
}

pub async fn update_folder(Json(payload): Json<UpdateFolderRequest>) -> impl IntoResponse {
    if let Err(e) = database::update_folder(payload) {
        eprintln!("Failed to update folder: {}", e);
        return Err(StatusCode::INTERNAL_SERVER_ERROR);
    }

    Ok(StatusCode::OK)
}

pub async fn update_bookmark(Json(payload): Json<UpdateBookmarkRequest>) -> impl IntoResponse {
    if let Err(e) = database::update_bookmark(payload) {
        eprintln!("Failed to update bookmark: {}", e);
        return Err(StatusCode::INTERNAL_SERVER_ERROR);
    }

    Ok(StatusCode::OK)
}
