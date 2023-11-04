use actix_web::cookie::{Cookie, self};
use actix_web::{post, web, HttpResponse};
use argon2::password_hash::rand_core::OsRng;
use argon2::password_hash::SaltString;
use argon2::{Argon2, PasswordHasher, PasswordHash, PasswordVerifier};
use chrono::{Duration, Utc};
use diesel::prelude::*;
use jsonwebtoken::{Header, EncodingKey};

use crate::jwt_auth::TokenClaims;
use crate::models::user::{NewUser, User};
use crate::schema::users::dsl::*;
use crate::AppState;

use serde_json::json;

#[post("/auth/register")]
async fn register(data: web::Json<NewUser>, app_state: web::Data<AppState>) -> HttpResponse {
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
        .execute(&mut db)
    {
        Ok(_) => HttpResponse::Ok().json(json!({
            "status": "success",
        })),
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
