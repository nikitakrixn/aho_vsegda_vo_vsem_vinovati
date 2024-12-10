use serde::Deserialize;
use std::env;

#[derive(Debug, Deserialize)]
pub struct Config {
    pub app_name: String,
    pub app_host: String,
    pub app_port: u16,
    pub database_url: String,
}

impl Config {
    pub fn load() -> Result<Self, env::VarError> {
        dotenv::dotenv().ok();

        let app_name = env::var("APP_NAME").unwrap_or_else(|_| "Ne backend".to_string());
        let app_host = env::var("APP_HOST").unwrap_or_else(|_| "127.0.0.1".to_string());
        let app_port = env::var("APP_PORT")
            .map(|s| s.parse().expect("APP_PORT must be a number"))
            .unwrap_or(3000);
        let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");

        Ok(Config {
            app_name,
            app_host,
            app_port,
            database_url,
        })
    }
}