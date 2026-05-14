use lumina_framework::prelude::*;

mod routes;

#[tokio::main]
async fn main() {
    // 1. Check for CLI arguments (for shell-out commands)
    let args: Vec<String> = std::env::args().collect();
    if args.len() > 1 {
        handle_cli_commands(&args).await;
        return;
    }

    // 2. Initialize Routers
    let web_router = routes::web::register();
    let api_router = routes::api::register();

    // 3. Start Application
    let app = Application::new()
        .with_web(web_router)
        .with_api(api_router);

    println!("🚀 Lumina Skeleton is starting...");
    app.serve("127.0.0.1:8000").await;
}

async fn handle_cli_commands(args: &[String]) {
    match args[1].as_str() {
        "migrate" => {
            println!("🔄 Running migrations...");
            // sqlx::migrate!("./database/migrations").run(&pool).await...
            // Note: User needs to implement this in their app
        }
        _ => println!("❓ Unknown command: {}", args[1]),
    }
}
