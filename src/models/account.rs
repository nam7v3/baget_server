use diesel::{Insertable, Queryable, Selectable};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Queryable, Selectable, Insertable, Serialize, Deserialize, Clone)]
#[diesel(table_name = crate::schema::account_table)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Account {
    #[diesel(column_name = "account_id")]
    pub id: i32,
    pub user_id: Uuid,
    #[diesel(column_name = "account")]
    pub account: String,
    pub balance: f32,
    pub income: f32,
    pub expense: f32,
}

#[derive(Queryable, Insertable, Serialize, Deserialize, Clone)]
#[diesel(table_name = crate::schema::account_table)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct SyncAccount {
    #[diesel(column_name = "account_id")]
    pub id: i32,
    #[diesel(column_name = "account")]
    pub accountType: String,
    pub balance: f32,
    pub income: f32,
    pub expense: f32,
}

impl SyncAccount {
    pub fn into_insert(&self, user_id: &Uuid) -> Account {
        Account {
            id: self.id,
            user_id: user_id.clone(),
            account: self.accountType.clone(),
            balance: self.balance,
            income: self.income,
            expense: self.expense,
        }
    }
}
