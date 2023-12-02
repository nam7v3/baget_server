use actix_web::cookie::{self, Cookie};
use actix_web::{post, web, HttpResponse};
use argon2::password_hash::rand_core::OsRng;
use argon2::password_hash::SaltString;
use argon2::{Argon2, PasswordHash, PasswordHasher, PasswordVerifier};
use chrono::{Duration, Utc};
use diesel::prelude::*;
use jsonwebtoken::{EncodingKey, Header};
use log::error;

use crate::jwt_auth::TokenClaims;
use crate::models::account::{Account, SyncAccount};
use crate::models::user::{NewUser, User};
use crate::schema::account_table::dsl::*;
use crate::schema::users::dsl::*;
use crate::AppState;

use serde_json::json;

#[post("/auth/register")]
async fn register(data: web::Json<NewUser>, app_state: web::Data<AppState>) -> HttpResponse {
    let DEFAULT_NEW_USER_ACCOUNT: Vec<SyncAccount> = vec![
        SyncAccount {
            id: 1,
            accountType: "Cash".to_owned(),
            balance: 0.0,
            income: 0.0,
            expense: 0.0,
        },
        SyncAccount {
            id: 2,
            accountType: "Bank".to_owned(),
            balance: 0.0,
            income: 0.0,
            expense: 0.0,
        },
        SyncAccount {
            id: 3,
            accountType: "Card".to_owned(),
            balance: 0.0,
            income: 0.0,
            expense: 0.0,
        },
    ];

    let mut db = app_state.db.get().unwrap();
    if let Ok(_) = users
        .filter(username.eq(&data.username))
        .select(username)
        .first::<String>(&mut db)
    {
        return HttpResponse::Conflict().json(json!({
            "status": "fail",
            "message": "Username already exists"
        }));
    }

    let salt = SaltString::generate(&mut OsRng);
    let hashed_password = Argon2::default()
        .hash_password(data.password.as_bytes(), &salt)
        .expect("Error while hashing password")
        .to_string();
    match diesel::insert_into(users)
        .values(NewUser {
            username: data.username.to_owned(),
            password: hashed_password,
        })
        .get_result::<User>(&mut db)
    {
        Ok(new_user) => {
            let new_account: Vec<Account> = DEFAULT_NEW_USER_ACCOUNT
                .clone()
                .iter()
                .map(|a| a.into_insert(&new_user.user_id))
                .collect();

            diesel::insert_into(account_table)
                .values(new_account)
                .execute(&mut db)
                .map_err(|err| {
                    error!("{}", err)
                })
                .unwrap();

            HttpResponse::Ok().json(json!({
                "status": "success",
            }))
        }
        Err(_) => HttpResponse::NotFound().json(json!({
            "status": "fail",
        })),
    }
}

#[post("/auth/login")]
async fn login(data: web::Json<NewUser>, app_state: web::Data<AppState>) -> HttpResponse {
    let mut db = app_state.db.get().unwrap();

    let user = match users
        .filter(username.eq(&data.username))
        .first::<User>(&mut db)
    {
        Ok(user) => user,
        Err(_) => {
            return HttpResponse::BadRequest()
                .json(json!({"status": "fail", "message": "Invalid username or password"}));
        }
    };

    let parsed_hash = PasswordHash::new(&user.password).unwrap();
    let valid = Argon2::default().verify_password(data.password.as_bytes(), &parsed_hash);

    if valid.is_err() {
        return HttpResponse::BadRequest().json(
            serde_json::json!({"status": "fail", "message": "Invalid username or password"}),
        );
    }

    let now = Utc::now();
    let iat = now.timestamp() as usize;
    let exp = (now + Duration::minutes(60)).timestamp() as usize;
    let claims: TokenClaims = TokenClaims {
        sub: user.user_id.to_string(),
        exp,
        iat,
    };

    let token = jsonwebtoken::encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(app_state.jwt_secret.as_ref()),
    )
    .unwrap();

    let cookie = Cookie::build("token", token.to_owned())
        .path("/")
        .max_age(cookie::time::Duration::new(60 * 60, 0))
        .http_only(true)
        .finish();

    HttpResponse::Ok()
        .cookie(cookie)
        .json(serde_json::json!({"status": "success", "token": token}))
}
