mod models;

use axum::{
    routing::{get, post},
    Json, Router,
};
use models::{Bookmark, Folder, FolderNode};
use serde_json::json;
use std::net::SocketAddr;
use time;
use tower_http::cors::{Any, CorsLayer};

async fn get_folders() -> Vec<Folder> {
    vec![
        Folder {
            id: 1,
            name: "Favorites".to_string(),
            parent_id: None,
        },
        Folder {
            id: 2,
            name: "Work".to_string(),
            parent_id: Some(1),
        },
    ]
}

async fn get_bookmarks() -> Vec<Bookmark> {
    vec![
        Bookmark {
            id: 1,
            name: "Rust Documentation".to_string(),
            url: "https://www.rust-lang.org".to_string(),
            favicon: "".to_string(),
            created: time::OffsetDateTime::now_utc(),
            folder_id: 1,
        },
        Bookmark {
            id: 2,
            name: "SvelteKit Docs".to_string(),
            url: "https://kit.svelte.dev".to_string(),
            favicon: "".to_string(),
            created: time::OffsetDateTime::now_utc(),
            folder_id: 2,
        },
    ]
}

async fn get_folder_tree() -> Json<Vec<FolderNode>> {
    let folders = get_folders().await;
    let bookmarks = get_bookmarks().await;

    let tree = Folder::to_tree(&folders, &bookmarks);

    Json(tree)
}

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let router = Router::new()
        .route("/api/folder-tree", get(get_folder_tree))
        .layer(CorsLayer::new().allow_origin(Any));

    let server = "0.0.0.0:3096";
    let listener = tokio::net::TcpListener::bind(&server).await?;
    // info!("Server running on {}", &server);
    let service = router.into_make_service_with_connect_info::<std::net::SocketAddr>();
    axum::serve(listener, service).await?;
    Ok(())
}
