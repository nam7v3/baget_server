use actix_web::error::ErrorBadRequest;
use actix_web::{get, post, web, HttpMessage, HttpRequest, Result, Responder};
use diesel::pg::upsert::*;
use uuid::Uuid;

use log::error;

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
) -> Result<impl Responder> {
    let ext = req.extensions();
    let uid = ext.get::<Uuid>().unwrap();
    let mut db = app_data.db.get().unwrap();

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
        .values(&transactions)
        .on_conflict(transaction_table.primary_key())
        .do_update()
        .set((
            entry_date.eq(excluded(entry_date)),
            amount.eq(excluded(amount)),
            transaction_table::account.eq(excluded(transaction_table::account)),
            category.eq(excluded(category)),
            transaction_type.eq(excluded(transaction_type)),
            transaction_title.eq(excluded(transaction_title)),
        ))
        .execute(&mut db)
        .map_err(|err|{
            error!("{}", err);
            ErrorBadRequest(err)
        })?;

    diesel::insert_into(account_table)
        .values(&expenses)
        .on_conflict(account_table.primary_key())
        .do_update()
        .set((
             account_table::account.eq(excluded(account_table::account)),
             balance.eq(excluded(balance)),
             income.eq(excluded(income)),
             expense.eq(excluded(expense))
        ))
        .execute(&mut db)
        .map_err(|err|{
            error!("{}", err);
            ErrorBadRequest(err)
        })?;
        

    diesel::insert_into(schedule_table)
        .values(&schedules)
        .on_conflict(schedule_table.primary_key())
        .do_update()
        .set((
            time_schedule.eq(excluded(time_schedule)),
            time_unit.eq(excluded(time_unit)),
            last_time_added.eq(excluded(last_time_added)),
        ))
        .execute(&mut db)
        .map_err(|err|{
            error!("{}", err);
            ErrorBadRequest(err)
        })?;
        
    Ok(web::Json(r#"{"status": "success"}"#))
}

#[get("/sync")]
pub async fn sync_get(
    req: HttpRequest,
    app_state: web::Data<AppState>,
    _: JwtMiddleware,
) -> Result<impl Responder>{
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
        .map_err(|err|{
            error!("{}", err);
            ErrorBadRequest(err)
        })?;

    let accounts = account_table
        .filter(account_table::user_id.eq(&uid))
        .select((account_id, account_table::account, balance, income, expense))
        .load::<SyncAccount>(&mut db)
        .map_err(|err|{
            error!("{}", err);
            ErrorBadRequest(err)
        })?;

    let schedules = schedule_table
        .filter(schedule_table::user_id.eq(&uid))
        .select((transaction_id, time_schedule, time_unit, last_time_added))
        .load::<SyncSchedule>(&mut db)
        .map_err(|err|{
            error!("{}", err);
            ErrorBadRequest(err)
        })?;

    Ok(web::Json(serde_json::json!({
        "status": "success",
        "data": RemoteSyncData{
            accounts,
            transactions,
            schedules
        }
    })))
}
