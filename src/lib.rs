use std::process;

pub fn set_nodename(args: Vec<String>) -> String {
    match args.len() {
      // no arguments passed
      1 => {
        let nodename = String::from("default");
        println!("Node not passed, setting nodename to: {}", nodename);
        return nodename;
      },
      // one argument passed
      2 => {
        let nodename = args[1].to_string();
        println!("Node passed: {}", nodename);
        return nodename;
      },
      // all the other cases
      _ => {
        //println!("This program takes only one (nodename) argument. Exiting (1)");
        process::exit(1);
      }
    }
  }
