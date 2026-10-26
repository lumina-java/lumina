use lumina::prelude::*;
use crate::app::controllers::welcome_controller::WelcomeController;

//                            WEB ROUTING
// Rute untuk web browser (HTML views, Blade templates, auth UI).

pub fn router() -> Router<AppState> {
    Router::new()
        // Halaman Utama Lumina (Dual Mode: HTML Dashboard di browser / JSON di API)
        .get("/", WelcomeController::index)
        
        // Rute autentikasi
        .get("/login", WelcomeController::login)
        .get("/register", WelcomeController::register)
}
