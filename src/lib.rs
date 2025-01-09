use log::{debug, error};
use dotenv::dotenv;

pub fn set_nodename(args: Vec<String>) -> (String, i32) {
  let mut nodename: String = "default".to_string();
  let mut returncode: i32 = 0;

  match args.len() {
    // no arguments passed
    1 => {
      debug!("Node not passed, using nodename: 'default'");
    },
    // one argument passed
    2 => {
      nodename = args[1].to_string();
      debug!("Node passed: {}", nodename);
    },
    // all the other cases
    _ => {
      error!("This program takes only one (nodename) argument. Exiting (1)");
      returncode = 1;
    }
  }

  let node_tuple: (String, i32) = (nodename, returncode);

  return node_tuple;
}

pub fn get_dburl() -> String {
  dotenv().ok();
  let database_url: String = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set.");
  return database_url;
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_set_nodename_no_args() {
    let args: Vec<String> = vec!["puppet_enc".to_string()];
    let node_tuple: (String, i32) = set_nodename(args);

    let nodename: &str = &node_tuple.0;
    let returncode: i32 = node_tuple.1;
    assert_eq!(nodename, ("default".to_string()));
    assert_eq!(returncode, (0));
  }

  #[test]
  fn test_set_nodename_with_arg() {
    let args: Vec<String> = vec!["puppet_enc".to_string(), "node1".to_string()];
    let node_tuple: (String, i32) = set_nodename(args);

    let nodename: &str = &node_tuple.0;
    let returncode: i32 = node_tuple.1;
    assert_eq!(nodename, ("node1".to_string()));
    assert_eq!(returncode, (0));
  }

  #[test]
  fn test_set_nodename_too_many_args() {
    let args: Vec<String> = vec!["puppet_enc".to_string(), "node1".to_string(), "extra_arg".to_string()];
    let node_tuple: (String, i32) = set_nodename(args);

    let returncode: i32 = node_tuple.1;
    assert_eq!(returncode, (1));
  }
}
