use std::sync::Arc;

use axum::{extract::State, http::StatusCode, response::IntoResponse, Json};

use crate::{
    database::{self, Pool},
    models::{UpdateBookmarkRequest, UpdateFolderRequest},
};

pub async fn favorite_bookmark(
    State(pool): State<Arc<Pool>>,
    Json(payload): Json<i32>,
) -> impl IntoResponse {
    let mut connection = pool.get().expect("Failed to get connection from pool");
    if let Err(e) = database::toggle_bookmark_favorite(&mut connection, payload) {
        eprintln!("Failed to toggle favorite: {}", e);
        return Err(StatusCode::INTERNAL_SERVER_ERROR);
    }

    Ok(StatusCode::OK)
}

pub async fn update_folder(
    State(pool): State<Arc<Pool>>,
    Json(payload): Json<UpdateFolderRequest>,
) -> impl IntoResponse {
    let mut connection = pool.get().expect("Failed to get connection from pool");
    if let Err(e) = database::update_folder(&mut connection, payload) {
        eprintln!("Failed to update folder: {}", e);
        return Err(StatusCode::INTERNAL_SERVER_ERROR);
    }

    Ok(StatusCode::OK)
}

pub async fn update_bookmark(
    State(pool): State<Arc<Pool>>,
    Json(payload): Json<UpdateBookmarkRequest>,
) -> impl IntoResponse {
    let mut connection = pool.get().expect("Failed to get connection from pool");
    if let Err(e) = database::update_bookmark(&mut connection, payload) {
        eprintln!("Failed to update bookmark: {}", e);
        return Err(StatusCode::INTERNAL_SERVER_ERROR);
    }

    Ok(StatusCode::OK)
}

pub async fn delete_folder(
    State(pool): State<Arc<Pool>>,
    Json(payload): Json<i32>,
) -> impl IntoResponse {
    let mut connection = pool.get().expect("Failed to get connection from pool");
    if let Err(e) = database::delete_folder(&mut connection, payload) {
        eprintln!("Failed to delete folder: {}", e);
        return Err(StatusCode::INTERNAL_SERVER_ERROR);
    }

    Ok(StatusCode::OK)
}

pub async fn delete_bookmark(
    State(pool): State<Arc<Pool>>,
    Json(payload): Json<i32>,
) -> impl IntoResponse {
    let mut connection = pool.get().expect("Failed to get connection from pool");
    if let Err(e) = database::delete_bookmark(&mut connection, payload) {
        eprintln!("Failed to delete bookmark: {}", e);
        return Err(StatusCode::INTERNAL_SERVER_ERROR);
    }

    Ok(StatusCode::OK)
}
