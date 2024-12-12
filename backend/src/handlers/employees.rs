use axum::{extract::{Path, State}, http::StatusCode, Json};
use diesel::{ExpressionMethods, QueryDsl};
use diesel_async::RunQueryDsl;
use crate::{db::DbPool, log_error, log_info, models::employees::{Employee, NewEmployee, UpdateEmployee}, schema::employees, services::ldap_service::{connect_to_ldap, LdapConfig}};



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

pub async fn trigger_account_creation(
    State(pool): State<DbPool>,
    Path(id): Path<i32>,
) -> Result<StatusCode, (StatusCode, String)> {
    log_info!("Triggering account creation for employee with id: {}", id);

    let mut conn = pool.get().await.map_err(|e| {
        log_error!("Database connection error: {}", e);
        (StatusCode::INTERNAL_SERVER_ERROR, e.to_string())
    })?;

    let config = LdapConfig::load().map_err(|e| {
        log_error!("Failed to load LDAP config: {:?}", e);
        (StatusCode::INTERNAL_SERVER_ERROR, e.to_string())
    })?;

    let mut ldap = connect_to_ldap(&config).await.map_err(|e| {
        log_error!("Failed to connect to LDAP: {:?}", e);
        (StatusCode::INTERNAL_SERVER_ERROR, "LDAP connection failed".to_string())
    })?;

    let employee = employees::table
        .filter(employees::id.eq(id))
        .first::<Employee>(&mut conn)
        .await
        .map_err(|e| {
            log_error!("Failed to get employee details: {:?}", e);
            (StatusCode::INTERNAL_SERVER_ERROR, "Failed to fetch employee".to_string())
        })?;

    let employee_email = employee.email.ok_or_else(|| {
        log_info!("No email found for employee with id: {}", id);
        (StatusCode::NOT_FOUND, "Employee email not found".to_string())
    })?;

    let search_result = ldap.search(
        &config.base_dn,
        ldap3::Scope::Subtree,
        &format!("(mail={})", employee_email),
        vec!["dn", "sAMAccountName"]
    ).await.map_err(|e| {
        log_error!("LDAP search failed: {:?}", e);
        (StatusCode::INTERNAL_SERVER_ERROR, "LDAP search failed".to_string())
    })?;

    let search_results = search_result.success().map_err(|e| {
        log_error!("LDAP operation failed: {:?}", e);
        (StatusCode::INTERNAL_SERVER_ERROR, "LDAP operation failed".to_string())
    })?;

    for entry in search_results.0 {
        let entry = ldap3::SearchEntry::construct(entry);

        let sam_account_name = entry.attrs.get("sAMAccountName")
            .and_then(|v| v.get(0))
            .map(|v| v.to_string())
            .unwrap_or_else(|| "Unknown".to_string());

        let update_result = diesel::update(employees::table.filter(employees::id.eq(id)))
            .set(employees::ad_login.eq(&sam_account_name))
            .execute(&mut conn)
            .await;

        match update_result {
            Ok(_) => {
                log_info!("ad_login updated for employee with id: {}", id);
                return Ok(StatusCode::OK);
            },
            Err(e) => {
                log_error!("Failed to update ad_login in DB: {:?}", e);
                return Err((StatusCode::INTERNAL_SERVER_ERROR, "Failed to update in DB".to_string()));
            }
        }
    }

    Err((StatusCode::NOT_FOUND, "LDAP entry not found for the given email".to_string()))
}