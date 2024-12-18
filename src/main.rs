mod db;
use puppet_enc::set_nodename;
use std::env;
use sqlx::{migrate::MigrateDatabase, Sqlite, Row, SqlitePool};
use log::debug;
use dotenv::dotenv;

#[async_std::main]
async fn main(){
  env_logger::init();

  let args: Vec<String> = env::args().collect();
  let nodename: String = set_nodename(args);
  debug!("final nodename: {}", nodename);

  dotenv().ok();
  let database_url: String = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set.");
  let db_url: &str = database_url.as_str();

  if !Sqlite::database_exists(db_url).await.unwrap_or(false) {
    Sqlite::create_database(db_url).await.unwrap();
    match db::create_schema(db_url).await{
      Ok(_) => debug!("Database created succesfully"),
      Err(e) => panic!("{}", e)
    }
    match db::insert_data(db_url).await{
      Ok(_) => debug!("Data added to database succesfully"),
      Err(e) => panic!("{}", e)
    }
  }
  let pool: sqlx::Pool<sqlx::Sqlite> = SqlitePool::connect(&db_url).await.unwrap();

  let node_info = sqlx::query("SELECT environment.name AS environment, role.name AS role
  FROM node
  INNER JOIN environment ON node.environment_id = environment.id
  INNER JOIN role ON node.role_id = role.id
  WHERE node.certname = $1
  ")
  .bind(nodename)
  .fetch_all(&pool)
  .await
  .unwrap();

  for row in node_info {
    let environment: String = row.get("environment");
    let role: String = row.get("role");
    println!("environment: {}, role: {}", environment, role);
  }

  pool.close().await;
  
}
