mod create;
mod database;
mod drag_drop;
mod export;
mod import;
mod models;
mod schema;
mod tree;

use axum::{
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
        .route("/api/import-bookmarks", post(import::import_bookmarks))
        .route("/api/export", get(export::export_bookmarks))
        .route("/api/create-folder", post(create::create_folder))
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
