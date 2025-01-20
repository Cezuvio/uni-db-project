mod db;

use actix_web::{
    delete, get, middleware, post, web, App, Error as AWError, Error, HttpResponse, HttpServer,
};
use db::{DbAction, Pool, Queries, Table};
use r2d2_sqlite::SqliteConnectionManager;
use std::io;

#[get("/tables")]
async fn handle_get_tables(pool: web::Data<Pool>) -> Result<HttpResponse, AWError> {
    let res = db::execute(&pool, Queries::GetAllTables).await;

    Ok(HttpResponse::Ok().json(res.map_err(AWError::from)?))
}

#[delete("/tables/{name}")]
pub async fn handle_delete_table(
    pool: web::Data<Pool>,
    name: web::Path<String>,
) -> Result<HttpResponse, Error> {
    let result = db::execute(&pool, Queries::DeleteTable(name.into_inner())).await?;

    match result.as_slice() {
        [] => Ok(HttpResponse::NotFound().finish()),
        [DbAction::Deleted] => Ok(HttpResponse::NoContent().finish()),
        _ => Ok(HttpResponse::InternalServerError().finish()),
    }
}

#[post("/table")]
pub async fn handle_create_table(
    form: web::Form<Table>,
    pool: web::Data<Pool>,
) -> Result<HttpResponse, Error> {
    let result = db::execute(&pool, Queries::CreateTable(form.name.clone())).await?;

    match result.as_slice() {
        [] => Ok(HttpResponse::Conflict().finish()),
        [DbAction::Created] => Ok(HttpResponse::Created().finish()),
        _ => Ok(HttpResponse::InternalServerError().finish()),
    }
}

#[actix_web::main]
async fn main() -> io::Result<()> {
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));

    let manager = SqliteConnectionManager::file("../../database.db");
    let pool = Pool::new(manager).expect("Database not found");

    {
        let conn = pool.get().expect("Failed to get database connection");
        conn.execute(
            r#"
        CREATE TABLE IF NOT EXISTS admins (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            name TEXT NOT NULL UNIQUE,
            password TEXT NOT NULL
        );
        "#,
            [],
        )
        .expect("Failed to initialize 'admins' table");
    }

    log::info!("starting HTTP server at https://localhost:8080");

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(pool.clone()))
            .wrap(middleware::Logger::default())
            .service(handle_get_tables)
            .service(handle_create_table)
            .service(handle_delete_table)
    })
    .bind(("0.0.0.0", 8080))?
    .workers(2)
    .run()
    .await
}
