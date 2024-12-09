use diesel::prelude::*;

#[derive(Queryable, Selectable)]
#[diesel(table_name = crate::schema::departments::table)]
pub struct Department {
    pub id: i32,
    pub name: String,
}

#[derive(Insertable)]
#[table_name = "departments"]
pub struct NewDepartment {
    pub name: String,
}