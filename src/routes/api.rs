use lumina_framework::prelude::*;

pub fn register() -> Router<AppState> {
    Router::new()
        .get("/health", || async { 
            serde_json::json!({
                "status": "up",
                "framework": "Lumina"
            }).to_string()
        })
}
