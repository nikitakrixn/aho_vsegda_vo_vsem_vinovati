use diesel::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Queryable, Selectable, Identifiable, Serialize, Deserialize, Debug, Clone)]
#[diesel(table_name = crate::schema::employees)]
pub struct Employee {
    pub id: i32,
    pub first_name: String,
    pub last_name: String,
    pub middle_name: Option<String>,
    pub department_id: i32,
    pub position_id: i32,
    pub hire_date: Option<chrono::NaiveDate>,
    pub termination_date: Option<chrono::NaiveDate>,
    pub ad_login: Option<String>,
    pub email: Option<String>,
    pub status: String,
    pub phone: Option<String>,
}

#[derive(Insertable, Deserialize, AsChangeset)]
#[diesel(table_name = crate::schema::employees)]
pub struct NewEmployee {
    pub first_name: String,
    pub last_name: String,
    pub middle_name: Option<String>,
    pub department_id: i32,
    pub position_id: i32,
    pub hire_date: Option<chrono::NaiveDate>,
    pub termination_date: Option<chrono::NaiveDate>,
    pub ad_login: Option<String>,
    pub email: Option<String>,
    pub status: String,
    pub phone: Option<String>,
}