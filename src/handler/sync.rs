use actix_web::{get, post, web, HttpMessage, HttpRequest, HttpResponse};
use uuid::Uuid;

use crate::jwt_auth::JwtMiddleware;
use crate::models::expense::{InsertExpense, QueryExpense};
use crate::models::transaction::{InsertTransaction, QueryTransaction};
use crate::models::TransactionExpenseInsert;
use crate::schema::expense_table::{self, dsl::*};
use crate::schema::transaction_table::{self, dsl::*};
use crate::AppState;
use diesel::prelude::*;

#[post("/sync")]
pub async fn sync_post(
    req: HttpRequest,
    data: web::Json<TransactionExpenseInsert>,
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

    diesel::delete(expense_table)
        .filter(expense_table::user_id.eq(&uid))
        .execute(&mut db)
        .unwrap();

    let transactions: Vec<InsertTransaction> = data
        .transactions
        .iter()
        .map(|t| t.into_insert(uid))
        .collect();

    let expenses: Vec<InsertExpense> = data.expenses.iter().map(|e| e.into_insert(uid)).collect();

    diesel::insert_into(transaction_table)
        .values(transactions)
        .execute(&mut db)
        .unwrap();

    diesel::insert_into(expense_table)
        .values(expenses)
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
            transaction_id,
            transaction_table::entry_date,
            transaction_table::amount,
            account,
            category,
            transaction_type,
            transaction_title,
        ))
        .load::<QueryTransaction>(&mut db)
        .unwrap();

    let expenses = expense_table
        .filter(expense_table::user_id.eq(&uid))
        .select((
            expense_id,
            expense_table::entry_date,
            expense_table::amount,
            expense,
        ))
        .load::<QueryExpense>(&mut db)
        .unwrap();

    HttpResponse::Ok().json(serde_json::json!({
        "status": "success",
        "data": {
            "transactions": transactions,
            "expenses": expenses,
        }
    }))
}
