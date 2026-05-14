use axum::{routing::get, Router};

pub fn init_routes() -> Router {
    Router::new()
        .route("/", get(|| async { "Welcome to Lumina Framework!" }))
        // ─── Protected Routes (do not remove this comment)
        
        // Debug (do not remove this comment)
}
