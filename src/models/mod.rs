use std::fmt::Display;

use serde::{Deserialize, Serialize};

use self::account::SyncAccount;
use self::transaction::SyncTransaction;

pub mod account;
pub mod transaction;
pub mod user;

#[derive(Deserialize, Serialize)]
pub struct TransactionExpenseInsert {
    pub accounts: Vec<SyncAccount>,
    pub transactions: Vec<SyncTransaction>,
}
