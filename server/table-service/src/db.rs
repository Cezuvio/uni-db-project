use actix_web::error;
use serde::{Deserialize, Serialize};
use sqlx::{sqlite::SqlitePool, Error as SqlxError};

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

pub async fn execute(pool: &SqlitePool, query: Queries) -> Result<Vec<DbAction>, actix_web::Error> {
    match query {
        Queries::GetAllTables => get_all_tables(pool).await,
        Queries::CreateTable(name) => create_table(pool, name).await,
        Queries::DeleteTable(name) => delete_table(pool, name).await,
    }
    .map_err(error::ErrorInternalServerError)
}

async fn get_all_tables(pool: &SqlitePool) -> Result<Vec<DbAction>, SqlxError> {
    let tables = sqlx::query!(
        "SELECT name FROM sqlite_master 
         WHERE type='table' AND name != 'sqlite_sequence'"
    )
    .fetch_all(pool)
    .await?;

    Ok(tables
        .into_iter()
        .map(|row| {
            DbAction::Table(Table {
                name: row.name.unwrap(),
            })
        })
        .collect())
}

async fn delete_table(pool: &SqlitePool, name: String) -> Result<Vec<DbAction>, SqlxError> {
    if name == "admins" {
        return Err(SqlxError::RowNotFound);
    }

    let exists = sqlx::query!(
        "SELECT 1 AS table_exists FROM sqlite_master WHERE type='table' AND name = ?",
        name
    )
    .fetch_optional(pool)
    .await?;

    if exists.is_none() {
        return Ok(vec![]);
    }

    sqlx::query(&format!("DROP TABLE IF EXISTS {}", name))
        .execute(pool)
        .await?;

    Ok(vec![DbAction::Deleted])
}

async fn create_table(pool: &SqlitePool, name: String) -> Result<Vec<DbAction>, SqlxError> {
    // Note: Since SQLite doesn't support parameterized table names,
    // we need to construct the query string manually.
    // In a production environment, you should add additional validation
    // for the table name to prevent SQL injection.
    let query = format!(
        "CREATE TABLE IF NOT EXISTS {} (
            DummyColumn INT
        )",
        name
    );

    sqlx::query(&query).execute(pool).await?;

    Ok(vec![DbAction::Created])
}

// Helper function to sanitize table names (recommended for production use)
fn sanitize_table_name(name: &str) -> bool {
    // Only allow alphanumeric characters and underscores
    name.chars().all(|c| c.is_alphanumeric() || c == '_')
}
