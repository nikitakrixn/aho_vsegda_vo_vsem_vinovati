use diesel::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Queryable, Selectable, Identifiable, Serialize, Deserialize, Debug, Clone)]
#[diesel(table_name = crate::schema::departments)]
pub struct Department {
    pub id: i32,
    pub name: String,
}

#[derive(Insertable, Deserialize, AsChangeset)]
#[diesel(table_name = crate::schema::departments)]
pub struct NewDepartment {
    pub name: String,
}