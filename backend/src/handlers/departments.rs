use axum::{extract::State, http::StatusCode, Json};
use diesel::{ExpressionMethods, QueryDsl};
use diesel_async::RunQueryDsl;

use crate::{db::DbPool, log_error, models::departments::Department, schema::departments};

pub async fn list_departments(
    State(pool): State<DbPool>,
) -> Result<Json<Vec<Department>>, StatusCode> {
    let mut conn = pool.get().await.map_err(|e| {
        log_error!("Database connection error: {}", e);
        StatusCode::INTERNAL_SERVER_ERROR
    })?;

    let departments = departments::table
        .order_by(departments::name.asc())
        .load::<Department>(&mut conn)
        .await
        .map_err(|e| {
            log_error!("Failed to load departments: {}", e);
            StatusCode::INTERNAL_SERVER_ERROR
        })?;

    Ok(Json(departments))
}