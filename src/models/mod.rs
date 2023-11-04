use serde::{Serialize, Deserialize};

use self::expense::SyncExpense;
use self::transaction::SyncTransaction;

pub mod user;
pub mod transaction;
pub mod expense;


#[derive(Deserialize, Serialize)]
pub struct TransactionExpenseInsert{
    pub transactions: Vec<SyncTransaction>,
    pub expenses: Vec<SyncExpense>,
}
