use axum::{extract::State, http::StatusCode, Json};
use diesel::{ExpressionMethods, QueryDsl};
use diesel_async::RunQueryDsl;

use crate::{db::DbPool, log_error, models::positions::Position, schema::positions};

pub async fn list_positions(
    State(pool): State<DbPool>,
) -> Result<Json<Vec<Position>>, StatusCode> {
    let mut conn = pool.get().await.map_err(|e| {
        log_error!("Database connection error: {}", e);
        StatusCode::INTERNAL_SERVER_ERROR
    })?;

    let positions = positions::table
        .order_by(positions::id.asc())
        .load::<Position>(&mut conn)
        .await
        .map_err(|e| {
            log_error!("Failed to load positions: {}", e);
            StatusCode::INTERNAL_SERVER_ERROR
        })?;

    Ok(Json(positions))
}