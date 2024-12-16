mod db;
use puppet_enc::set_nodename;

use std::env;

use sqlx::{migrate::MigrateDatabase, Sqlite};

use log::debug;

const DB_URL: &str = "sqlite://db/puppet_enc.sqlite?mode=rwc";

#[async_std::main]
async fn main(){
  env_logger::init();

  let args: Vec<String> = env::args().collect();
  let nodename: String = set_nodename(args);
  debug!("final nodename: {}", nodename);
  
  if !Sqlite::database_exists(DB_URL).await.unwrap_or(false) {
    Sqlite::create_database(DB_URL).await.unwrap();
    match db::create_schema(DB_URL).await{
      Ok(_) => debug!("Database created succesfully"),
      Err(e) => panic!("{}", e)
    }
    match db::insert_data(DB_URL).await{
      Ok(_) => debug!("Data added to database succesfully"),
      Err(e) => panic!("{}", e)
    }
  }
}
