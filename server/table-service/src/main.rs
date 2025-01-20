mod db;

use actix_web::{
    delete, get, middleware, post, web, App, Error as AWError, HttpResponse, HttpServer,
};
use db::{DbAction, Queries, Table};
use sqlx::mysql::{MySqlPool, MySqlPoolOptions};
use std::io;

#[get("/tables")]
async fn handle_get_tables(pool: web::Data<MySqlPool>) -> Result<HttpResponse, AWError> {
    let res = db::execute(&pool, Queries::GetAllTables).await;
    Ok(HttpResponse::Ok().json(res.map_err(AWError::from)?))
}

#[delete("/tables/{name}")]
pub async fn handle_delete_table(
    pool: web::Data<MySqlPool>,
    name: web::Path<String>,
) -> Result<HttpResponse, AWError> {
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
    pool: web::Data<MySqlPool>,
) -> Result<HttpResponse, AWError> {
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

    let pool = MySqlPoolOptions::new()
        .max_connections(5)
        .connect(&std::env::var("DATABASE_URL").expect("DATABASE_URL env var not set"))
        .await
        .unwrap();

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
