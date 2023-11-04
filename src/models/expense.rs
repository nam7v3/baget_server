use diesel::{Queryable, Selectable, Insertable};
use serde::{Serialize, Deserialize};
use uuid::Uuid;

#[derive(Queryable, Selectable, Insertable, Serialize, Deserialize, Clone)]
#[diesel(table_name = crate::schema::expense_table)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Expense{
    pub expense_id: i32,
    pub user_id: Uuid,
    pub entry_date: String,
    pub amount: f32,
    pub expense: String,
}

#[derive(Queryable, Selectable, Serialize, Deserialize, Clone)]
#[diesel(table_name = crate::schema::expense_table)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct QueryExpense{
    pub expense_id: i32,
    pub entry_date: String,
    pub amount: f32,
    pub expense: String,
}

#[derive(Insertable, Serialize, Deserialize, Clone)]
#[diesel(table_name = crate::schema::expense_table)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct InsertExpense{
    pub user_id: Uuid,
    pub entry_date: String,
    pub amount: f32,
    pub expense: String,
}


#[derive(Insertable, Serialize, Deserialize, Clone)]
#[diesel(table_name = crate::schema::expense_table)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct SyncExpense{
    pub entry_date: String,
    pub amount: f32,
    pub expense: String,
}


impl SyncExpense {
    pub fn into_insert(&self, user_id: &Uuid) -> InsertExpense {
        InsertExpense {
            user_id: user_id.clone(),
            entry_date: self.entry_date.clone(),
            amount: self.amount.clone(),
            expense: self.expense.clone(),
        }
    }
}
