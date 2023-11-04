use actix_web::middleware::Logger;
use actix_web::{web, App, HttpServer};

use baget_server::handler::config;
use baget_server::{AppState, Pool};

use diesel::r2d2::ConnectionManager;
use diesel::{r2d2, PgConnection};

use dotenvy::dotenv;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));

    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let manager = ConnectionManager::<PgConnection>::new(database_url);
    let jwt_secret = std::env::var("JWT_SECRET").expect("JWT_SECRET must be set");
    let pool: Pool = r2d2::Pool::builder()
        .build(manager)
        .expect("Failed to create pool");
    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(AppState {
                db: pool.clone(),
                jwt_secret: jwt_secret.clone(),
            }))
            .configure(config)
            .wrap(Logger::default())
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
