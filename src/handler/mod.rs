use actix_web::web;

pub mod auth;
pub mod sync;

use auth::{login, register};
use sync::{sync_get, sync_post};

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(login)
        .service(register)
        .service(sync_get)
        .service(sync_post);
}
