use actix_web::{get, post, web, HttpMessage, HttpRequest, HttpResponse};
use uuid::Uuid;

use crate::jwt_auth::JwtMiddleware;
use crate::models::account::{Account, SyncAccount};
use crate::models::schedule::{SyncSchedule, Schedule};
use crate::models::transaction::{SyncTransaction, Transaction};
use crate::models::RemoteSyncData;
use crate::schema::account_table::{self, dsl::*};
use crate::schema::transaction_table::{self, dsl::*};
use crate::schema::schedule_table::{self, dsl::*};
use crate::AppState;
use diesel::prelude::*;

#[post("/sync")]
pub async fn sync_post(
    req: HttpRequest,
    data: web::Json<RemoteSyncData>,
    app_data: web::Data<AppState>,
    _: JwtMiddleware,
) -> HttpResponse {
    let ext = req.extensions();
    let uid = ext.get::<Uuid>().unwrap();
    let mut db = app_data.db.get().unwrap();
    diesel::delete(transaction_table)
        .filter(transaction_table::user_id.eq(&uid))
        .execute(&mut db)
        .unwrap();

    diesel::delete(account_table)
        .filter(account_table::user_id.eq(&uid))
        .execute(&mut db)
        .unwrap();

    diesel::delete(schedule_table)
        .filter(schedule_table::user_id.eq(&uid))
        .execute(&mut db)
        .unwrap();

    let transactions: Vec<Transaction> = data
        .transactions
        .iter()
        .map(|t| t.into_insert(uid))
        .collect();

    let expenses: Vec<Account> = data.accounts.iter().map(|e| e.into_insert(uid)).collect();
    let schedules: Vec<Schedule> = data
        .schedules
        .iter()
        .map(|t| t.into_insert(uid))
        .collect();

    diesel::insert_into(transaction_table)
        .values(transactions)
        .execute(&mut db)
        .unwrap();

    diesel::insert_into(account_table)
        .values(expenses)
        .execute(&mut db)
        .unwrap();

    diesel::insert_into(schedule_table)
        .values(schedules)
        .execute(&mut db)
        .unwrap();

    HttpResponse::Ok().json(serde_json::json!({
        "status": "success"
    }))
}

#[get("/sync")]
pub async fn sync_get(
    req: HttpRequest,
    app_state: web::Data<AppState>,
    _: JwtMiddleware,
) -> HttpResponse {
    let ext = req.extensions();
    let uid = ext.get::<Uuid>().unwrap();
    let mut db = app_state.db.get().unwrap();

    let transactions = transaction_table
        .filter(transaction_table::user_id.eq(&uid))
        .select((
            _timestamp,
            transaction_table::entry_date,
            transaction_table::amount,
            transaction_table::account,
            category,
            transaction_type,
            transaction_title,
        ))
        .load::<SyncTransaction>(&mut db)
        .unwrap();

    let accounts = account_table
        .filter(account_table::user_id.eq(&uid))
        .select((account_id, account_table::account, balance, income, expense))
        .load::<SyncAccount>(&mut db)
        .unwrap();

    let schedules = schedule_table
        .filter(schedule_table::user_id.eq(&uid))
        .select((transaction_id, time_schedule, time_unit, last_time_added))
        .load::<SyncSchedule>(&mut db)
        .unwrap();

    HttpResponse::Ok().json(serde_json::json!({
        "status": "success",
        "data": RemoteSyncData{
            accounts,
            transactions,
            schedules
        }
    }))
}
