use std::sync::Arc;

use axum::{extract::State, http::StatusCode, response::IntoResponse, Json};
use serde::Deserialize;
use serde_json::json;

use crate::database::{self, Pool};

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

pub async fn create_folder(
    State(pool): State<Arc<Pool>>,
    Json(payload): Json<NewFolderRequest>,
) -> impl IntoResponse {
    let mut connection = pool.get().expect("Failed to get connection from pool");
    match database::create_new_folder(&mut connection, payload.name, payload.parent_id) {
        Ok(id) => Ok(Json(json!({ "id": id }))),
        Err(e) => {
            eprintln!("Failed to create folder: {}", e);
            return Err(StatusCode::INTERNAL_SERVER_ERROR);
        }
    }
}

pub async fn create_bookmark(
    State(pool): State<Arc<Pool>>,
    Json(payload): Json<NewBookmarkRequest>,
) -> impl IntoResponse {
    let mut connection = pool.get().expect("Failed to get connection from pool");
    match database::create_new_bookmark(
        &mut connection,
        payload.name,
        payload.url,
        payload.folder_id,
    ) {
        Ok(id) => Ok(Json(json!({ "id": id }))),
        Err(e) => {
            eprintln!("Failed to create bookmark: {}", e);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}
