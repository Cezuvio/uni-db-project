mod db;

use actix_web::{middleware, post, web, App, Error as AWError, HttpResponse, HttpServer};
use serde::Deserialize;
use sha2::{Digest, Sha256};
use sqlx::sqlite::SqlitePoolOptions;
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

    let pool = SqlitePoolOptions::new()
        .max_connections(5)
        .connect(&std::env::var("DATABASE_URL").unwrap())
        .await
        .unwrap();

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
