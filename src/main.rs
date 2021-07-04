pub mod cmd;
pub mod create_cmd;
pub mod utils;

use clap::App;
use clap::Arg;
use create_cmd::CreateCommand;
use std::io::Result;

static VERSION: &'static str = "1.0.0";

#[cfg(target_os = "linux")]
fn main() -> Result<()> {
    use crate::cmd::CommandRunner;

    let app = App::new("machinery")
        .version(VERSION)
        .subcommand(App::new("version").about("prints the current version of the app"))
        .subcommand(
            App::new("create")
                .about("creates either a worker or a node")
                .subcommand(
                    App::new("worker").arg(
                        Arg::with_name("number")
                            .long("--number")
                            .short("-n")
                            .takes_value(true)
                            .max_values(1),
                    ),
                )
                .subcommand(App::new("worker")),
        );

    let matches = app.get_matches();
    match matches.subcommand() {
        ("version", Some(_)) => println!("this app version is {}", &VERSION),
        ("create", Some(args)) => return CreateCommand::new().run(&args),
        _ => panic!("not able to parse command"),
    }

    return Ok(());
}
