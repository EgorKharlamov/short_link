use crate::schema::links;
use chrono;
use chrono::prelude::*;
use diesel::prelude::*;

#[derive(Queryable, Clone)]
pub struct Link {
    pub id: i32,
    pub shared_link: String,
    pub original_link: String,
    pub created_at: NaiveDateTime,
}

#[derive(Insertable)]
#[diesel(table_name = links)]
pub struct NewLink<'a> {
    pub shared_link: String,
    pub original_link: &'a str,
    pub created_at: &'a NaiveDateTime,
}
