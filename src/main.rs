mod db;
use puppet_enc::{get_dburl, set_nodename};
use std::env;
use sqlx::{migrate::MigrateDatabase, Sqlite, Row, SqlitePool};
use log::debug;
use serde::{Serialize, Deserialize};
use serde_yml;

#[derive(Serialize, Deserialize, PartialEq, Debug)]
struct NodeInfo {
  environment: String,
  role: String
}

#[async_std::main]
async fn main() {
  env_logger::init();
  
  let args: Vec<String> = env::args().collect();
  let _returncode: i32 = 0;
  let node_tuple: (String, i32) = set_nodename(args);

  let nodename: &str = &node_tuple.0;
  let returncode: i32 = node_tuple.1;

  debug!("final node_tuple: {:#?}", node_tuple);
  debug!("final nodename: {}", nodename);
  debug!("final returncode: {}", returncode);
  
  if returncode != 0 {
    std::process::exit(returncode);
  }

  let database_url: String = get_dburl();
  let db_url = &database_url.as_str();
  
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
    
    let node_info = NodeInfo {
      environment: {environment},
      role: {role}
    };
    
    // Serialize to YAML
    let yaml = serde_yml::to_string(&node_info);
    println!("yaml: {:?}", yaml);
  }
  
  pool.close().await;
  
}
