use serde::{Deserialize, Serialize};

use self::account::SyncAccount;
use self::schedule::SyncSchedule;
use self::transaction::SyncTransaction;

pub mod account;
pub mod transaction;
pub mod user;
pub mod schedule;

#[derive(Deserialize, Serialize)]
pub struct RemoteSyncData {
    pub accounts: Vec<SyncAccount>,
    pub transactions: Vec<SyncTransaction>,
    pub schedules: Vec<SyncSchedule>,
}
