use lumina::prelude::*;
use serde_json::json;

//                            API ROUTING
// Rute di sini otomatis diprefix dengan "/api" oleh Application engine.
// Sangat ideal untuk membangun RESTful API, Microservices, dan Webhooks.

pub fn router() -> Router<AppState> {
    Router::new()
        .get("/", api_index)
        .get("/health", api_health)
        .get("/v1", api_v1_index)
}

/// Endpoint: GET /api
async fn api_index(req: Request) -> LuminaResult {
    req.ok(json!({
        "framework": "Lumina Framework",
        "tagline": "The High-Performance Rust Backend for Modern APIs",
        "version": "0.3.0",
        "status": "online",
        "endpoints": {
            "health": "/api/health",
            "v1": "/api/v1"
        }
    }))
}

/// Endpoint: GET /api/health
async fn api_health(req: Request) -> LuminaResult {
    let db_status = if let Some(pool) = &req.state.db {
        if sqlx::query("SELECT 1").execute(&pool.pool).await.is_ok() {
            "connected"
        } else {
            "error"
        }
    } else {
        "disabled"
    };

    req.ok(json!({
        "status": "ok",
        "database": db_status,
        "timestamp": chrono::Utc::now().to_rfc3339()
    }))
}

/// Endpoint: GET /api/v1
async fn api_v1_index(req: Request) -> LuminaResult {
    req.ok(json!({
        "version": "v1",
        "message": "Lumina API v1 is ready. Register your controllers in src/routes/api.rs!"
    }))
}
