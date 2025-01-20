use actix_web::{get, middleware, web, App, Error as AWError, HttpResponse, HttpServer};
use sqlx::sqlite::{SqlitePool, SqlitePoolOptions};
use std::io;

#[get("/tables/{name}")]
pub async fn handle_get_table(
    pool: web::Data<SqlitePool>,
    name: web::Path<String>,
) -> Result<HttpResponse, AWError> {
    Ok(HttpResponse::Ok().finish())
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
            .service(handle_get_table)
    })
    .bind(("0.0.0.0", 8080))?
    .workers(2)
    .run()
    .await
}
