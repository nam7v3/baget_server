use chrono::NaiveDateTime;
use diesel::{Insertable, Queryable, Selectable, sql_types::Timestamp};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Queryable, Selectable, Insertable, Deserialize, Serialize)]
#[diesel(table_name = crate::schema::transaction_table)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Transaction {
    #[diesel(column_name = "_timestamp")]
    pub date: i32,
    pub user_id: Uuid,
    pub entry_date: String,
    pub amount: f32,
    pub account: String,
    pub category: String,
    pub transaction_type: String,
    pub transaction_title: String,
}

#[derive(Queryable, Selectable, Deserialize, Serialize)]
#[diesel(table_name = crate::schema::transaction_table)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct SyncTransaction {
    #[diesel(column_name = "_timestamp")]
    pub date: i32,
    pub entry_date: String,
    pub amount: f32,
    pub account: String,
    pub category: String,
    pub transaction_type: String,
    pub transaction_title: String,
}

impl SyncTransaction {
    pub fn into_insert(&self, user_id: &Uuid) -> Transaction {
        Transaction {
            date: self.date,
            user_id: user_id.clone(),
            entry_date: self.entry_date.clone(),
            amount: self.amount,
            account: self.account.clone(),
            category: self.category.clone(),
            transaction_type: self.transaction_type.clone(),
            transaction_title: self.transaction_title.clone(),
        }
    }
}
