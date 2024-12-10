mod config;
mod logging;

use std::net::SocketAddr;

use axum::Router;
use axum::routing::get;
use diesel::{r2d2::{self, ConnectionManager}, PgConnection};
use tower_http::cors::{AllowOrigin, CorsLayer};

pub type DbPool = r2d2::Pool<ConnectionManager<PgConnection>>;

async fn create_db_pool(config: &config::Config) -> DbPool {
    let manager = ConnectionManager::<PgConnection>::new(&config.database_url);
    r2d2::Pool::builder()
        .build(manager)
        .expect("Failed to create pool.")
}

#[tokio::main]
async fn main() {
    logging::init_logging();

    let config = config::Config::load().expect("Failed to load configuration");
    tracing::info!("Loaded configuration: {:?}", config);

    let db_pool = create_db_pool(&config).await;

    let addr_string = format!("{}:{}", config.app_host, config.app_port);
    let addr = addr_string.parse::<SocketAddr>().expect(&format!("Can't parse {}", addr_string));

    let app = Router::new().route("/", get(|| async { "Hello, world!" }).layer(CorsLayer::new().allow_origin(AllowOrigin::mirror_request())));

    let listener = tokio::net::TcpListener::bind(&addr).await.unwrap();

    axum::serve(listener, app)
        .await
        .unwrap();
}