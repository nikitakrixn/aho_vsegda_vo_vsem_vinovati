use diesel::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Queryable, Selectable)]
#[diesel(table_name = crate::schema::employees)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Employee {
    pub id: i32,
    pub first_name: String,
    pub last_name: String,
    pub middle_name: Option<String>,
    pub department_id: Option<i32>,
    pub position_id: Option<i32>,
    pub hire_date: chrono::NaiveDate,
    pub termination_date: Option<chrono::NaiveDate>,
    pub ad_login: Option<String>,
    pub email: Option<String>,
    pub status: String,
    pub phone: Option<String>,
}

#[derive(Debug, Deserialize, Insertable)]
#[diesel(table_name = crate::schema::employees)]
pub struct NewEmployee {
    pub first_name: String,
    pub last_name: String,
    pub middle_name: Option<String>,
    pub department_id: Option<i32>,
    pub position_id: Option<i32>,
    pub hire_date: chrono::NaiveDate,
    pub ad_login: Option<String>,
    pub email: Option<String>,
    pub status: String,
    pub phone: Option<String>,
}

#[derive(Debug, Deserialize, AsChangeset)]
#[diesel(table_name = crate::schema::employees)]
pub struct UpdateEmployee {
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub middle_name: Option<String>,
    pub department_id: Option<i32>,
    pub position_id: Option<i32>,
    pub email: Option<String>,
    pub phone: Option<String>,
}