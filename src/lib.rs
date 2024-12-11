use std::process;
use log::{debug, error};

pub fn set_nodename(args: Vec<String>) -> String {
    match args.len() {
      // no arguments passed
      1 => {
        let nodename = String::from("default");
        debug!("Node not passed, setting nodename to: {}", nodename);
        return nodename;
      },
      // one argument passed
      2 => {
        let nodename = args[1].to_string();
        debug!("Node passed: {}", nodename);
        return nodename;
      },
      // all the other cases
      _ => {
        error!("This program takes only one (nodename) argument. Exiting (1)");
        process::exit(1);
      }
    }
  }
