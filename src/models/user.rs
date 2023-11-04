use chrono::NaiveDateTime;
use uuid::Uuid;
use serde::Serialize;
use serde::Deserialize;
use diesel::{Insertable, Selectable, Queryable};

#[derive(Queryable, Selectable, Insertable, Deserialize, Serialize)]
#[diesel(table_name = crate::schema::users)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct User{
    pub user_id: Uuid,
    pub username: String,
    pub password: String,
    pub verified: bool,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Insertable, Deserialize, Serialize)]
#[diesel(table_name = crate::schema::users)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct NewUser{
    pub username: String,
    pub password: String,
}
