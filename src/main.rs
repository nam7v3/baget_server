use actix_web::middleware::Logger;
use actix_web::{web, App, HttpServer};

use baget_server::handler::config;
use baget_server::{AppState, Pool};

use diesel::pg::Pg;
use diesel::r2d2::ConnectionManager;
use diesel::{r2d2, PgConnection};
use diesel_migrations::{embed_migrations, EmbeddedMigrations, MigrationHarness};

use dotenvy::dotenv;

pub const MIGRATIONS: EmbeddedMigrations = embed_migrations!();

pub fn run_db_migrations(conn: &mut impl MigrationHarness<Pg>) {
    conn.run_pending_migrations(MIGRATIONS).expect("Could not run migrations");
}

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
    
    run_db_migrations(&mut pool.get().unwrap());

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(AppState {
                db: pool.clone(),
                jwt_secret: jwt_secret.clone(),
            }))
            .configure(config)
            .wrap(Logger::default())
    })
    .bind(("0.0.0.0", 8080))?
    .run()
    .await
}
