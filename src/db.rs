
use std::result::Result;
use sqlx::{sqlite::SqliteQueryResult, SqlitePool};

pub async fn create_schema(db_url:&str) -> Result<SqliteQueryResult, sqlx::Error>{
    let pool = SqlitePool::connect(&db_url).await?;
    let result = sqlx::query_file!("db/schema.sql").execute(&pool).await;
    pool.close().await;
    return result;
}
