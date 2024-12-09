
use std::result::Result;
use sqlx::{sqlite::SqliteQueryResult, SqlitePool};

pub async fn create_schema(db_url:&str) -> Result<SqliteQueryResult, sqlx::Error>{
    let pool = SqlitePool::connect(&db_url).await?;
    let qry =
    "PRAGMA foreign_keys = ON;
    CREATE TABLE IF NOT EXISTS environment (
        id INTEGER PRIMARY KEY AUTOINCREMENT,
        name VARCHAR(50) UNIQUE,
        description TEXT,
        created_on DATETIME DEFAULT (datetime('now', 'utc')),
        updated_on DATETIME DEFAULT (datetime('now', 'utc'))
    );
    CREATE TABLE IF NOT EXISTS role (
        id INTEGER PRIMARY KEY AUTOINCREMENT,
        name VARCHAR(50),
        description TEXT,
        created_on DATETIME DEFAULT (datetime('now', 'utc')),
        updated_on DATETIME DEFAULT (datetime('now', 'utc'))
    );
    CREATE TABLE IF NOT EXISTS node (
        id INTEGER PRIMARY KEY AUTOINCREMENT,
        certname VARCHAR(50),
        environment_id INTEGER,
        role_id INTEGER,
        created_on DATETIME DEFAULT (datetime('now', 'utc')),
        updated_on DATETIME DEFAULT (datetime('now', 'utc')),
        FOREIGN KEY (environment_id) REFERENCES node (environment_id),
        FOREIGN KEY (role_id) REFERENCES node (role_id)
    );";
    let result = sqlx::query(&qry).execute(&pool).await;
    pool.close().await;
    return result;
}
