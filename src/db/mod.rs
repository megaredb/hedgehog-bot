pub mod users;

use diesel_async::{pooled_connection::AsyncDieselConnectionManager, AsyncPgConnection};

pub type Pool = bb8::Pool<AsyncDieselConnectionManager<AsyncPgConnection>>;
pub type Connection<'a> =
    bb8::PooledConnection<'a, AsyncDieselConnectionManager<AsyncPgConnection>>;
