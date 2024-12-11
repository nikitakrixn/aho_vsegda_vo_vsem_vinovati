use axum::{extract::{Path, State}, http::StatusCode, Json};
use diesel::{ExpressionMethods, QueryDsl};
use diesel_async::RunQueryDsl;
use crate::{db::DbPool, log_error, log_info, models::employees::{Employee, NewEmployee, UpdateEmployee}, schema::employees};



pub async fn list_employees(State(pool): State<DbPool>) -> Result<Json<Vec<Employee>>, StatusCode> {
    log_info!("Listing all employees");

    let mut conn = pool.get().await.map_err(|e| {
        log_error!("Database connection error: {}", e);
        StatusCode::INTERNAL_SERVER_ERROR
    })?;

    let results = employees::table.load::<Employee>(&mut conn).await 
        .map_err(|e| {
            log_error!("Failed to load employees: {}", e);
            StatusCode::INTERNAL_SERVER_ERROR
        })?;

    Ok(Json(results))
}

pub async fn get_employee(
    State(pool): State<DbPool>,
    Path(id): Path<i32>,
) -> Result<(StatusCode, Json<Employee>), StatusCode> {
    log_info!("Fetching employee with id: {}", id);

    let mut conn = pool.get().await.map_err(|e| {
        log_error!("Database connection error: {}", e);
        StatusCode::INTERNAL_SERVER_ERROR
    })?;

    let employee = employees::table
        .find(id)
        .first::<Employee>(&mut conn)
        .await
        .map_err(|e| {
            log_error!("Failed to fetch employee {}: {}", id, e);
            match e {
                diesel::result::Error::NotFound => StatusCode::NOT_FOUND,
                _ => StatusCode::INTERNAL_SERVER_ERROR,
            }
        })?;

    log_info!("Successfully fetched employee with id: {}", id);
    Ok((StatusCode::OK, Json(employee)))
}


pub async fn create_employee(
    State(pool): State<DbPool>,
    Json(new_employee): Json<NewEmployee>,
) -> Result<(StatusCode, Json<Employee>), StatusCode> {
    log_info!("Creating new employee: {:?}", new_employee);

    let mut conn = pool.get().await.map_err(|e| {
        log_error!("Database connection error: {}", e);
        StatusCode::INTERNAL_SERVER_ERROR
    })?;

    let employee: Employee = diesel::insert_into(employees::table)
        .values(&new_employee)
        .get_result(&mut conn)
        .await
        .map_err(|e| {
            log_error!("Failed to create employee: {}", e);
            StatusCode::INTERNAL_SERVER_ERROR
        })?;

    log_info!("Successfully created employee with id: {}", employee.id);
    Ok((StatusCode::CREATED, Json(employee)))
}

pub async fn update_employee(
    State(pool): State<DbPool>,
    Path(id): Path<i32>,
    Json(update_employee): Json<UpdateEmployee>,
) -> Result<(StatusCode, Json<Employee>), StatusCode> {
    log_info!("Updating employee with id {}: {:?}", id, update_employee);

    let mut conn = pool.get().await.map_err(|e| {
        log_error!("Database connection error: {}", e);
        StatusCode::INTERNAL_SERVER_ERROR
    })?;

    let employee: Employee = diesel::update(employees::table.find(id))
        .set(&update_employee)
        .get_result(&mut conn)
        .await
        .map_err(|e| {
            log_error!("Failed to update employee {}: {}", id, e);
            match e {
                diesel::result::Error::NotFound => StatusCode::NOT_FOUND,
                _ => StatusCode::INTERNAL_SERVER_ERROR,
            }
        })?;

    log_info!("Successfully updated employee with id: {}", employee.id);
    Ok((StatusCode::OK, Json(employee)))
}

pub async fn delete_employee(
    State(pool): State<DbPool>,
    Path(id): Path<i32>,
) -> Result<StatusCode, StatusCode> {
    log_info!("Deleting (deactivating) employee with id: {}", id);

    let mut conn = pool.get().await.map_err(|e| {
        log_error!("Database connection error: {}", e);
        StatusCode::INTERNAL_SERVER_ERROR
    })?;

    // Используем "мягкое" удаление, изменяя статус на "inactive"
    let result = diesel::update(employees::table.find(id))
        .set((
            employees::status.eq("уволен"),
            employees::termination_date.eq(chrono::Local::now().naive_local().date()),
        ))
        .execute(&mut conn)
        .await
        .map_err(|e| {
            log_error!("Failed to delete employee {}: {}", id, e);
            match e {
                diesel::result::Error::NotFound => StatusCode::NOT_FOUND,
                _ => StatusCode::INTERNAL_SERVER_ERROR,
            }
        })?;

    if result == 0 {
        log_error!("Employee with id {} not found", id);
        return Err(StatusCode::NOT_FOUND);
    }

    log_info!("Successfully deleted (deactivated) employee with id: {}", id);
    Ok(StatusCode::NO_CONTENT)
}

pub async fn hard_delete_employee(
    State(pool): State<DbPool>,
    Path(id): Path<i32>,
) -> Result<StatusCode, StatusCode> {
    log_info!("Hard deleting employee with id: {}", id);

    let mut conn = pool.get().await.map_err(|e| {
        log_error!("Database connection error: {}", e);
        StatusCode::INTERNAL_SERVER_ERROR
    })?;

    let result = diesel::delete(employees::table.find(id))
        .execute(&mut conn)
        .await
        .map_err(|e| {
            log_error!("Failed to hard delete employee {}: {}", id, e);
            match e {
                diesel::result::Error::NotFound => StatusCode::NOT_FOUND,
                _ => StatusCode::INTERNAL_SERVER_ERROR,
            }
        })?;

    if result == 0 {
        log_error!("Employee with id {} not found", id);
        return Err(StatusCode::NOT_FOUND);
    }

    log_info!("Successfully hard deleted employee with id: {}", id);
    Ok(StatusCode::NO_CONTENT)
}