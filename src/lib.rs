pub mod models;
pub mod schema;
pub mod jwt_auth;
pub mod handler;


use diesel::{r2d2::{self, ConnectionManager}, PgConnection};
pub type Pool = r2d2::Pool<ConnectionManager<PgConnection>>;

pub struct AppState{
    pub db: Pool,
    pub jwt_secret: String,
}
