use actix_web::{error, web};
use serde::{Deserialize, Serialize};

pub type Pool = r2d2::Pool<r2d2_sqlite::SqliteConnectionManager>;
pub type Connection = r2d2::PooledConnection<r2d2_sqlite::SqliteConnectionManager>;

#[derive(Debug, Serialize, Deserialize)]
pub struct Table {
    pub name: String,
}

#[allow(clippy::enum_variant_names)]
pub enum Queries {
    GetAllTables,
    GetAllRows(String),
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

pub async fn execute(pool: &Pool, query: Queries) -> Result<Vec<DbAction>, actix_web::Error> {
    let pool = pool.clone();
    let conn = web::block(move || pool.get())
        .await?
        .map_err(error::ErrorInternalServerError)?;
    web::block(move || match query {
        Queries::GetAllTables => get_all_tables(conn),
        Queries::GetAllRows(name) => get_all_rows(conn, name),
        Queries::CreateTable(name) => create_table(conn, name),
        Queries::DeleteTable(name) => delete_table(conn, name),
    })
    .await?
    .map_err(error::ErrorInternalServerError)
}

fn get_all_tables(conn: Connection) -> Result<Vec<DbAction>, rusqlite::Error> {
    let mut stmt = conn.prepare(
        "SELECT name FROM sqlite_master 
         WHERE type='table' AND name != 'sqlite_sequence'",
    )?;

    stmt.query_map([], |row| Ok(DbAction::Table(Table { name: row.get(0)? })))
        .and_then(Iterator::collect)
}

fn delete_table(conn: Connection, name: String) -> Result<Vec<DbAction>, rusqlite::Error> {
    if name == "admins" {
        return Err(rusqlite::Error::InvalidQuery);
    }

    let exists: bool = conn
        .query_row(
            "SELECT 1 FROM sqlite_master WHERE type='table' AND name=?",
            [&name],
            |_| Ok(true),
        )
        .unwrap_or(false);

    if !exists {
        return Ok(vec![]);
    }

    conn.execute(&format!("DROP TABLE {}", name), [])?;
    Ok(vec![DbAction::Deleted])
}

fn create_table(conn: Connection, name: String) -> Result<Vec<DbAction>, rusqlite::Error> {
    conn.execute(
        &format!(
            "CREATE TABLE IF NOT EXISTS {} (
                DummyColumn INT
            );",
            name
        ),
        [],
    )?;

    Ok(vec![DbAction::Created])
}

fn get_all_rows(conn: Connection, table_name: String) -> Result<Vec<DbAction>, rusqlite::Error> {
    let exists: bool = conn
        .query_row(
            "SELECT 1 FROM sqlite_master WHERE type='table' AND name=?",
            [&table_name],
            |_| Ok(true),
        )
        .unwrap_or(false);

    if !exists {
        return Ok(vec![]);
    }

    let mut stmt = conn.prepare(&format!("SELECT json_object(*) FROM {}", table_name))?;
    let rows = stmt.query_map([], |row| {
        let json_str: String = row.get(0)?;
        Ok(DbAction::Row(json_str))
    })?;

    rows.collect()
}
