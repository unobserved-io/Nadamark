use axum::{http::StatusCode, response::IntoResponse, Json};

use crate::database;

pub async fn favorite_bookmark(Json(payload): Json<i32>) -> impl IntoResponse {
    if let Err(e) = database::toggle_bookmark_favorite(payload) {
        eprintln!("Failed to create folder: {}", e);
        return Err(StatusCode::INTERNAL_SERVER_ERROR);
    }

    Ok(StatusCode::OK)
}
