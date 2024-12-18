use std::result::Result;
use sqlx::{sqlite::SqliteQueryResult,Error, SqlitePool};

pub async fn create_schema(db_url:&str) -> Result<SqliteQueryResult, Error>{
    let pool: sqlx::Pool<sqlx::Sqlite> = SqlitePool::connect(&db_url).await?;
    let result = sqlx::query_file!("db/schema.sql").execute(&pool).await;
    pool.close().await;
    return result;
}

pub async fn insert_data(db_url:&str) -> Result<SqliteQueryResult, Error>{
    let pool: sqlx::Pool<sqlx::Sqlite> = SqlitePool::connect(&db_url).await?;
    let result = sqlx::query_file!("db/data.sql").execute(&pool).await;
    pool.close().await;
    return result;
}

/*
  let instances = SqlitePool::connect(&DB_URL).await.unwrap();
  let qry = "INSERT INTO environment (name, description) VALUES($1, $2)";
  let insert_query = sqlx::query(&qry).bind("test").bind("The example.com Testing Environment").execute(&instances).await;
  
  instances.close().await;
  info!("{:?}", insert_query);

  */