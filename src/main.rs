use std::env;
use std::result::Result;
use sqlx::{sqlite::SqliteQueryResult, Sqlite, SqlitePool, migrate::MigrateDatabase};

async fn create_schema(db_url:&str) -> Result<SqliteQueryResult, sqlx::Error>{
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

#[async_std::main]
async fn main(){
    let args: Vec<String> = env::args().collect();
    let request = &args[1];
    println!("Requested Node: {request}");

    let db_url = String::from("sqlite://db/puppet_enc.sqlite"); 
    if !Sqlite::database_exists(&db_url).await.unwrap_or(false){
        Sqlite::create_database(&db_url).await.unwrap();
        match create_schema(&db_url).await{
            Ok(_) => println!("Database created succesfully"),
            Err(e) => panic!("{}", e)

        }
    }
    let instances = SqlitePool::connect(&db_url).await.unwrap();
    let qry = "INSERT INTO environment (name, description) VALUES($1, $2)";
    let result = sqlx::query(&qry).bind("test").bind("The example.com Testing Environment").execute(&instances).await;

    instances.close().await;
    println!("{:?}", result);
}
