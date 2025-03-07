use actix_web::middleware::Logger;
use actix_web::{web, App, HttpServer, HttpResponse};
use env_logger::Env;
use std::process::exit;
use log::info;

mod apprise;
mod grafana;
mod routes;
mod state;
mod utils;

async fn handle_not_found() -> HttpResponse {
    info!("Received request to unknown route. Available routes are:");
    info!("POST /notify/{{key}} - Send notification (key is required, matches your Apprise configuration)");
    info!("GET  /health      - Health check endpoint");
    
    HttpResponse::NotFound()
        .content_type("text/plain")
        .body("404 Not Found\n\nAvailable routes:\n\nPOST /notify/{key} - Send notification (key is required, matches your Apprise configuration)\nGET  /health      - Health check endpoint\n")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::Builder::from_env(Env::default().default_filter_or("info")).init();

    let apprise_url = match apprise::get_apprise_url() {
        Some(h) => h,
        None => {
            log::error!("Invalid apprise host");
            exit(1);
        }
    };

    let app_state = state::AppState { apprise_url };

    HttpServer::new(move || {
        App::new()
            .wrap(Logger::default())
            .data(app_state.clone())
            .route("/notify/{key}", web::post().to(routes::notify))
            .route("/health", web::get().to(routes::health))
            .default_service(web::route().to(handle_not_found))
    })
    .workers(utils::get_workers())
    .bind(format!("0.0.0.0:{}", utils::get_port()))?
    .run()
    .await
}
