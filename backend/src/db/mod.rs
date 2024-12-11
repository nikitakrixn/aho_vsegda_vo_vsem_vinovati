use diesel_async::{
    pooled_connection::{deadpool::Pool, AsyncDieselConnectionManager},
    AsyncPgConnection,
};

pub type DbPool = Pool<AsyncPgConnection>;

pub fn create_pool(database_url: &str) -> DbPool {
    let config = AsyncDieselConnectionManager::<AsyncPgConnection>::new(database_url);
    Pool::builder(config)
        .build()
        .expect("Failed to create pool")
}