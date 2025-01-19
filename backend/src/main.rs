mod create;
mod database;
mod drag_drop;
mod export;
mod import;
mod models;
mod modify;
mod schema;
mod tree;

use axum::{
    extract::DefaultBodyLimit,
    http,
    routing::{get, post},
    Router,
};
use tower_http::cors::{Any, CorsLayer};

#[tokio::main]
async fn main() -> std::io::Result<()> {
    database::initialize_database();

    let router = Router::new()
        .route("/api/folder-tree", get(tree::refresh_tree))
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
        // 20 MB File limit
        .layer(DefaultBodyLimit::max(1024 * 1024 * 20))
        .layer(
            CorsLayer::new()
                .allow_origin(Any)
                .allow_headers([http::header::CONTENT_TYPE]),
        );

    let server = "0.0.0.0:3096";
    let listener = tokio::net::TcpListener::bind(&server).await?;
    // info!("Server running on {}", &server);
    let service = router.into_make_service_with_connect_info::<std::net::SocketAddr>();
    axum::serve(listener, service).await?;

    Ok(())
}
