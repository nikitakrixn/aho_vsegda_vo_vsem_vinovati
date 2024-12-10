mod conifg;
mod logging;

use std::net::SocketAddr;

use axum::Router;
use axum::routing::get;
use diesel::{r2d2::{self, ConnectionManager}, PgConnection};
use tower_http::cors::{AllowOrigin, CorsLayer};

pub type DbPool = r2d2::Pool<ConnectionManager<PgConnection>>;

async fn create_db_pool() -> DbPool {
    dotenv::dotenv().ok();

    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let manager = ConnectionManager::<PgConnection>::new(database_url);
    r2d2::Pool::builder()
        .build(manager)
        .expect("Failed to create pool.")
}

#[tokio::main]
async fn main() {
    logging::init_logging();

    let config = conifg::Config::load().expect("Failed to load configuration");

    let db_pool = create_db_pool().await;

    let conn = db_pool.get().expect("Can't get db connection from pool");

    let addr_string = format!("{}:{}", config.app_host, config.app_port);
    let addr = addr_string.parse::<SocketAddr>().expect(&format!("Can't parse {}", addr_string));

    let app = Router::new().route("/", get(|| async { "Hello, world!" }).layer(CorsLayer::new().allow_origin(AllowOrigin::mirror_request())));

    let listener = tokio::net::TcpListener::bind(&addr).await.unwrap();

    axum::serve(listener, app)
        .await
        .unwrap();
}