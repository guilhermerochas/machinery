use std::{io::Result, process::Command};

use clap::ArgMatches;

pub struct CreateCommand;

impl CreateCommand {
    pub fn new() -> CreateCommand {
        return CreateCommand;
    }

    pub fn run(&self, opts: &clap::ArgMatches) -> Result<()> {
        match opts.subcommand() {
            ("manager", Some(args)) => self.run_worker(args),
            _ => println!("no subcommands were provided or subcommand not found!"),
        }

        return Ok(());
    }

    fn run_worker(&self, args: &ArgMatches) {
        let num_nodes: &str;

        match args.args.get("number") {
            Some(value) => num_nodes = value.vals[0].to_str().unwrap(),
            None => num_nodes = "0",
        }

        match num_nodes.parse::<i64>() {
            Ok(nodes) => {
                let command = Command::new("sh")
                    .arg("-c")
                    .arg(format!("ls -l | head -n {}", nodes))
                    .output()
                    .expect("failed to execute process");

                println!(
                    "command result {}",
                    std::str::from_utf8(&command.stdout).unwrap()
                )
            }
            Err(_) => println!("The value provided is not an integer"),
        }
    }
}
