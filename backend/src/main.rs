mod config;
mod db;
mod logging;
mod handlers;
mod models;
mod schema;

use std::net::SocketAddr;
use axum::{routing::{delete, get, post, put}, Router};
use tower_http::cors::{AllowOrigin, CorsLayer};

#[tokio::main]
async fn main() {
    let config = config::Config::load().expect("Failed to load configuration");

    logging::init_logging(&config.app_name);

    let db_pool = db::create_pool(&config.database_url);

    let addr_string = format!("{}:{}", config.app_host, config.app_port);
    let addr = addr_string.parse::<SocketAddr>().expect(&format!("Can't parse {}", addr_string));

    let app = Router::new()
        .route("/employees", post(handlers::employees::create_employee))
        .route("/employees", get(handlers::employees::list_employees))
        .route("/employees/:id", get(handlers::employees::get_employee))
        .route("/employees/:id", put(handlers::employees::update_employee))
        .route("/employees/:id", delete(handlers::employees::delete_employee))
        .route("/employees/:id/hard", delete(handlers::employees::hard_delete_employee))
        .route("/departments", get(handlers::departments::list_departments))
        .route("/positions", get(handlers::positions::list_positions))
        .layer(CorsLayer::new().allow_origin(AllowOrigin::mirror_request()))
        .with_state(db_pool);

    tracing::info!("Listening on {}", addr);
    let listener = tokio::net::TcpListener::bind(&addr).await.unwrap();

    axum::serve(listener, app)
        .await
        .unwrap();
}