mod db;

use std::env;
use sqlx::{Sqlite, SqlitePool, migrate::MigrateDatabase};

#[async_std::main]
async fn main(){
    let args: Vec<String> = env::args().collect();
    let request = &args[1];
    println!("Requested Node: {request}");

    let db_url = String::from("sqlite://db/puppet_enc.sqlite"); 
    if !Sqlite::database_exists(&db_url).await.unwrap_or(false){
        Sqlite::create_database(&db_url).await.unwrap();
        match db::create_schema(&db_url).await{
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
