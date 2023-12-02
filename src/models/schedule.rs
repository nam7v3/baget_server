use diesel::{Insertable, Queryable, Selectable};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Queryable, Selectable, Insertable, Serialize, Deserialize, Clone)]
#[diesel(table_name = crate::schema::schedule_table)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Schedule {
    #[diesel(column_name = "transaction_id")]
    pub transactionDto: i64,
    #[diesel(column_name = "user_id")]
    pub user_id: Uuid,
    #[diesel(column_name = "time_schedule")]
    pub timeSchedule: i32,
    #[diesel(column_name = "time_unit")]
    pub timeUnit: String,
    #[diesel(column_name = "last_time_added")]
    pub lastTimeAdded: i64,
}

#[derive(Queryable, Insertable, Serialize, Deserialize, Clone)]
#[diesel(table_name = crate::schema::schedule_table)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct SyncSchedule {
    #[diesel(column_name = "transaction_id")]
    pub transactionDto: i64,
    #[diesel(column_name = "time_schedule")]
    pub timeSchedule: i32,
    #[diesel(column_name = "time_unit")]
    pub timeUnit: String,
    #[diesel(column_name = "last_time_added")] 
    pub lastTimeAdded: i64,
}

impl SyncSchedule {
    pub fn into_insert(&self, user_id: &Uuid) -> Schedule {
        Schedule { transactionDto: self.transactionDto, user_id: user_id.clone(), timeSchedule: self.timeSchedule, timeUnit: self.timeUnit.clone(), lastTimeAdded: self.lastTimeAdded }
    }
}
