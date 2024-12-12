mod config;
mod db;
mod logging;
mod handlers;
mod models;
mod schema;
mod services;

use std::{net::SocketAddr, time::Duration};
use axum::{http::{Request, Response}, routing::{delete, get, post}, Router};
use tower_http::{cors::{AllowOrigin, CorsLayer}, trace::TraceLayer};
use tracing::Span;

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
        .route("/employees/:id", 
                get(handlers::employees::get_employee)
                .put(handlers::employees::update_employee)
                .delete(handlers::employees::delete_employee))
        .route("/employees/:id/hard", delete(handlers::employees::hard_delete_employee))
        .route("/employees/:id/create_accounts", post(handlers::employees::trigger_account_creation))
        .route("/departments", get(handlers::departments::list_departments))
        .route("/positions", get(handlers::positions::list_positions))
        .layer(CorsLayer::new().allow_origin(AllowOrigin::mirror_request()))
        .layer(
            TraceLayer::new_for_http()
                .make_span_with(|request: &Request<_>| {
                    tracing::info_span!(
                        "http_request",
                        method = %request.method(),
                        uri = %request.uri(),
                        version = ?request.version(),
                    )
                })
                .on_request(|request: &Request<_>, _span: &Span| {
                    tracing::info!("started {} {}", request.method(), request.uri());
                })
                .on_response(|response: &Response<_>, latency: Duration, _span: &Span| {
                    tracing::info!(
                        status = response.status().as_u16(),
                        latency = ?latency,
                        "finished"
                    );
                })
        )
        .with_state(db_pool);

    tracing::info!("Listening on {}", addr);
    let listener = tokio::net::TcpListener::bind(&addr).await.unwrap();

    axum::serve(listener, app)
        .await
        .unwrap();
}