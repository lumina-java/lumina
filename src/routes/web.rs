use lumina_framework::prelude::*;

pub fn register() -> Router<AppState> {
    Router::new()
        .get("/", |state: AppState| async move {
            state.view.render("welcome", serde_json::json!({
                "app_name": "Lumina Framework",
                "version": "0.1.0"
            })).await
        })
}
