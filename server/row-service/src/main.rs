use actix_web::{get, middleware, web, App, Error, HttpResponse, HttpServer};
use r2d2_sqlite::SqliteConnectionManager;
use std::io;
pub type Pool = r2d2::Pool<r2d2_sqlite::SqliteConnectionManager>;

#[get("/tables/{name}")]
pub async fn handle_get_table(
    pool: web::Data<Pool>,
    name: web::Path<String>,
) -> Result<HttpResponse, Error> {
    Ok(HttpResponse::Ok().finish())
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
            .service(handle_get_table)
    })
    .bind(("0.0.0.0", 8080))?
    .workers(2)
    .run()
    .await
}
