use actix_web::{get, middleware, web, App, Error as AWError, HttpResponse, HttpServer};
use serde::{Deserialize, Serialize};
use sqlx::mysql::MySqlRow;
use sqlx::mysql::{MySqlPool, MySqlPoolOptions};
use sqlx::Row;
use std::io;

#[derive(Serialize, Deserialize)]
struct Column {
    column_name: String,
    data_type: String,
    column_default: Option<String>,
    is_nullable: String,
    is_unique: bool,
}

#[derive(Serialize, Deserialize)]
struct ColumnData {
    columns: Vec<Column>,
}

#[get("/tables/{name}")]
pub async fn handle_get_table(
    pool: web::Data<MySqlPool>,
    name: web::Path<String>,
) -> Result<HttpResponse, AWError> {
    // Sanitize table name to prevent SQL injection
    let table_name = name.into_inner();
    if !table_name.chars().all(|c| c.is_alphanumeric() || c == '_') {
        return Ok(HttpResponse::BadRequest().body("Invalid table name"));
    }

    let columns_query = format!(
        "SELECT 
             COLUMN_NAME,
             DATA_TYPE,
             COLUMN_DEFAULT,
             IS_NULLABLE,
             COLUMN_KEY
         FROM information_schema.columns
         WHERE table_name = ?;"
    );

    let columns: Vec<MySqlRow> = sqlx::query(&columns_query)
        .bind(&table_name)
        .fetch_all(pool.get_ref())
        .await
        .map_err(|e| {
            eprintln!("Failed to get columns: {}", e);
            HttpResponse::InternalServerError().body("Database error")
        })
        .unwrap();

    let column_data: Vec<Column> = columns
        .into_iter()
        .map(|row| Column {
            column_name: row.get("COLUMN_NAME"),
            data_type: row.get("DATA_TYPE"),
            column_default: row.get("COLUMN_DEFAULT"),
            is_nullable: row.get("IS_NULLABLE"),
            is_unique: row.get::<String, _>("COLUMN_KEY") == "UNI",
        })
        .collect();

    let json = serde_json::to_string(&column_data).unwrap();

    Ok(HttpResponse::Ok().json(json))
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
