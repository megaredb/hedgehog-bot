use diesel::{delete, prelude::*};
use diesel_async::RunQueryDsl;

use crate::{models::User, schema::users};

use super::Connection;

pub async fn update_user<'a>(conn: &mut Connection<'a>, user: User) -> QueryResult<User> {
    diesel::update(users::table)
        .filter(users::id.eq(user.id))
        .set(user)
        .get_result(conn)
        .await
}

pub async fn create_user<'a>(conn: &mut Connection<'a>, user: User) -> QueryResult<User> {
    diesel::insert_into(users::table)
        .values(user)
        .returning(User::as_returning())
        .get_result(conn)
        .await
}

pub async fn get_user<'a>(conn: &mut Connection<'a>, id: i64) -> QueryResult<User> {
    users::table.find(id).first(conn).await
}

pub async fn get_user_by_boosty_id<'a>(
    conn: &mut Connection<'a>,
    boosty_id: i64,
) -> QueryResult<User> {
    users::table
        .filter(users::boosty_id.eq(boosty_id))
        .first(conn)
        .await
}

pub async fn remove_user<'a>(conn: &mut Connection<'a>, user_id: i64) -> QueryResult<usize> {
    delete(users::table)
        .filter(users::id.eq(user_id))
        .execute(conn)
        .await
}

// pub async fn get_users<'a>(conn: &mut Connection<'a>) -> QueryResult<Vec<User>> {
//     users::table.load::<User>(conn).await
// }

pub async fn get_users_boosty_ids<'a>(conn: &mut Connection<'a>) -> QueryResult<Vec<i64>> {
    users::table.select(users::id).load::<i64>(conn).await
}
