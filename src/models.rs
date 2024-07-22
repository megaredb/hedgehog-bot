use chrono::NaiveDateTime;
use diesel::prelude::*;

#[derive(Queryable, Selectable, Identifiable, Insertable, AsChangeset)]
#[diesel(table_name = crate::schema::users)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct User {
    pub id: i64,
    pub boosty_id: i64,
    pub expires_at: NaiveDateTime,
}

// #[derive(Insertable)]
// #[diesel(table_name = crate::schema::users)]
// pub struct NewUser {
//     pub id: i64,
//     pub boosty_id: i64,
//     pub expires_at: NaiveDateTime,
// }
