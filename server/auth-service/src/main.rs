use actix_web::{middleware, post, web, App, Error as AWError, HttpResponse, HttpServer};
use r2d2::Pool;
use r2d2_sqlite::SqliteConnectionManager;
use serde::Deserialize;
use sha2::{Digest, Sha256};
use std::io;

#[derive(Deserialize)]
struct LoginRequest {
    name: String,
    password: String,
}

#[post("/login")]
async fn handle_login(form: web::Form<LoginRequest>) -> Result<HttpResponse, AWError> {
    let mut hasher = Sha256::new();
    hasher.update(form.password.clone());
    let result = hasher.finalize();
    let hex_string = hex::encode(result);

    Ok(HttpResponse::Ok().json(hex_string))
}

#[actix_web::main]
async fn main() -> io::Result<()> {
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));

    let manager = SqliteConnectionManager::file("../../database.db");
    let pool = Pool::new(manager).expect("Database not found");

    log::info!("starting HTTP server at https://localhost:8080");

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(pool.clone()))
            .wrap(middleware::Logger::default())
            .service(handle_login)
    })
    .bind(("0.0.0.0", 8080))?
    .workers(2)
    .run()
    .await
}
