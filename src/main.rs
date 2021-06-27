mod create;

use std::io::Result;

#[cfg(target_os = "linux")]
use clap::App;
use clap::Arg;
use create::CreateCommand;

static VERSION: &'static str = "1.0.0";

fn main() -> Result<()> {
    let app = App::new("machinery")
        .version(VERSION)
        .subcommand(App::new("version").about("prints the current version of the app"))
        .subcommand(
            App::new("create")
                .about("creates either a worker or a node")
                .subcommand(
                    App::new("manager").arg(
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
        _ => println!("not able to parse command"),
    }

    return Ok(());
}
