use serde::{Deserialize, Serialize};

use self::expense::SyncExpense;
use self::transaction::SyncTransaction;

pub mod expense;
pub mod transaction;
pub mod user;

#[derive(Deserialize, Serialize)]
pub struct TransactionExpenseInsert {
    pub transactions: Vec<SyncTransaction>,
    pub expenses: Vec<SyncExpense>,
}
