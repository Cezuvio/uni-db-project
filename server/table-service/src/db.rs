use actix_web::error;
use serde::{Deserialize, Serialize};
use sqlx::{mysql::MySqlPool, Error as SqlxError};

#[derive(Debug, Serialize, Deserialize)]
pub struct Table {
    pub name: String,
}

#[allow(clippy::enum_variant_names)]
pub enum Queries {
    GetAllTables,
    CreateTable(String),
    DeleteTable(String),
}

#[derive(Debug, Serialize, Deserialize)]
pub enum DbAction {
    Row(String),
    Table(Table),
    Created,
    Deleted,
    LoggedIn(String),
}

pub async fn execute(pool: &MySqlPool, query: Queries) -> Result<Vec<DbAction>, actix_web::Error> {
    match query {
        Queries::GetAllTables => get_all_tables(pool).await,
        Queries::CreateTable(name) => create_table(pool, name).await,
        Queries::DeleteTable(name) => delete_table(pool, name).await,
    }
    .map_err(error::ErrorInternalServerError)
}

async fn get_all_tables(pool: &MySqlPool) -> Result<Vec<DbAction>, SqlxError> {
    let tables = sqlx::query!(
        "SELECT TABLE_NAME AS name FROM information_schema.tables 
         WHERE table_schema = DATABASE()"
    )
    .fetch_all(pool)
    .await?;

    Ok(tables
        .into_iter()
        .map(|row| DbAction::Table(Table { name: row.name }))
        .collect())
}

async fn delete_table(pool: &MySqlPool, name: String) -> Result<Vec<DbAction>, SqlxError> {
    if !sanitize_table_name(&name) {
        return Err(SqlxError::RowNotFound);
    }

    if name == "admins" {
        return Err(SqlxError::RowNotFound);
    }

    let exists = sqlx::query!(
        "SELECT 1 AS table_exists FROM information_schema.tables 
         WHERE table_schema = DATABASE() AND TABLE_NAME = ?",
        name
    )
    .fetch_optional(pool)
    .await?;

    if exists.is_none() {
        return Ok(vec![]);
    }

    sqlx::query(&format!("DROP TABLE IF EXISTS `{}`", name))
        .execute(pool)
        .await?;

    Ok(vec![DbAction::Deleted])
}

async fn create_table(pool: &MySqlPool, name: String) -> Result<Vec<DbAction>, SqlxError> {
    if !sanitize_table_name(&name) {
        return Err(SqlxError::RowNotFound);
    }

    let query = format!(
        "CREATE TABLE IF NOT EXISTS `{}` (
            DummyColumn INT
        )",
        name
    );

    sqlx::query(&query).execute(pool).await?;

    Ok(vec![DbAction::Created])
}

fn sanitize_table_name(name: &str) -> bool {
    name.chars().all(|c| c.is_alphanumeric() || c == '_')
}
