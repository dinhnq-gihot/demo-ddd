use {
    diesel::prelude::*,
    serde::Serialize,
};

#[derive(Debug, Clone, Serialize, Identifiable, AsChangeset, Selectable, Queryable)]
#[diesel(table_name = crate::schema::users)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct User {
    pub id: i32,
    pub name: String,
    pub email: String,
}

#[derive(Debug, Insertable)]
#[diesel(table_name = crate::schema::users)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct NewUser<'a> {
    pub id: &'a i32,
    pub name: &'a str,
    pub email: &'a str,
}
