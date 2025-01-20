use actix_web::{get, middleware, web, App, Error as AWError, HttpResponse, HttpServer};
use serde::Serialize;
use sqlx::{
    mysql::{MySqlPool, MySqlPoolOptions},
    FromRow,
};
use std::io;

#[derive(FromRow, Serialize)]
struct RowData {
    id: i32,
    name: String,
}

#[get("/tables/{name}")]
pub async fn handle_get_table(
    pool: web::Data<MySqlPool>,
    name: web::Path<String>,
) -> Result<HttpResponse, AWError> {
    let table_name = name.into_inner();

    let query = format!("SELECT * FROM {}", table_name);

    let rows: Result<Vec<RowData>, sqlx::Error> =
        sqlx::query_as(&query).fetch_all(pool.get_ref()).await;

    match rows {
        Ok(rows) => Ok(HttpResponse::Ok().json(rows)),
        Err(_) => Ok(HttpResponse::InternalServerError().finish()),
    }
}

#[actix_web::main]
async fn main() -> io::Result<()> {
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));

    let pool = MySqlPoolOptions::new()
        .max_connections(5)
        .connect(&std::env::var("DATABASE_URL").expect("DATABASE_URL env var not set"))
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
