mod create;
mod database;
mod drag_drop;
mod export;
mod import;
mod models;
mod modify;
mod schema;
mod tree;

use std::sync::Arc;

use axum::{
    extract::DefaultBodyLimit,
    http,
    routing::{get, post},
    Router,
};
use tower_http::{
    cors::{Any, CorsLayer},
    services::ServeDir,
};

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let pool = Arc::new(database::initialize_database());

    let static_path = std::env::var("STATIC_FILES_PATH").unwrap_or_else(|_| {
        // Development path
        #[cfg(debug_assertions)]
        return "../frontend/build".to_string();
        // Production path
        #[cfg(not(debug_assertions))]
        return "./static".to_string();
    });

    let router =
        Router::new()
            .route("/api/tree", get(tree::refresh_tree))
            .route("/api/move", post(drag_drop::handle_move))
            .route("/api/import-html", post(import::import_bookmarks_html))
            .route(
                "/api/import-linkwarden",
                post(import::import_bookmarks_linkwarden),
            )
            .route("/api/export", get(export::export_bookmarks))
            .route("/api/create-folder", post(create::create_folder))
            .route("/api/create-bookmark", post(create::create_bookmark))
            .route("/api/favorite-bookmark", post(modify::favorite_bookmark))
            .route("/api/update-folder", post(modify::update_folder))
            .route("/api/update-bookmark", post(modify::update_bookmark))
            .route("/api/delete-folder", post(modify::delete_folder))
            .route("/api/delete-bookmark", post(modify::delete_bookmark))
            .with_state(pool)
            .fallback_service(ServeDir::new(&static_path).not_found_service(
                ServeDir::new(&static_path).append_index_html_on_directories(true),
            ))
            // 20 MB File upload limit
            .layer(DefaultBodyLimit::max(1024 * 1024 * 20))
            .layer(
                CorsLayer::new()
                    .allow_origin(Any)
                    .allow_headers([http::header::CONTENT_TYPE]),
            );

    let server = "0.0.0.0:8663";
    let listener = tokio::net::TcpListener::bind(&server).await?;
    let service = router.into_make_service_with_connect_info::<std::net::SocketAddr>();
    axum::serve(listener, service).await?;

    Ok(())
}
