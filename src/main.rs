mod db;
use puppet_enc::set_nodename;

use std::env;
use sqlx::migrate::MigrateDatabase;

#[async_std::main]
async fn main(){
  let args: Vec<String> = env::args().collect();
  let nodename: String = set_nodename(args);
  println!("final nodename: {}", nodename);
  
  let db_url = String::from("sqlite://db/puppet_enc.sqlite"); 
  
  if !sqlx::Sqlite::database_exists(&db_url).await.unwrap_or(false) {
    sqlx::Sqlite::create_database(&db_url).await.unwrap();
    match db::create_schema(&db_url).await{
      Ok(_) => println!("Database created succesfully"),
      Err(e) => panic!("{}", e)
    }
    match db::insert_data(&db_url).await{
      Ok(_) => println!("Data added to database succesfully"),
      Err(e) => panic!("{}", e)
    }
  }
}
