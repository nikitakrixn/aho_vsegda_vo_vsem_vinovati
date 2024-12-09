use diesel::prelude::*;

#[derive(Queryable, Selectable)]
#[diesel(table_name = )]
pub struct Position {
    pub id: i32,
    pub name: String,
}

#[derive(Insertable)]
#[diesel(table_name = "positions")]
pub struct NewPosition<'a> {
    pub name: &'a str,
}