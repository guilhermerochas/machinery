use crate::{cmd::CommandRunner, utils};
use core::{panic, str};
use std::{io::Result, sync::Arc, usize, vec};

pub struct CreateCommand;

impl CreateCommand {
    pub fn new() -> CreateCommand {
        return CreateCommand;
    }

    fn run_worker(&self, args: &clap::ArgMatches<'_>) {
        let num_nodes: &str;

        match args.args.get("number") {
            Some(value) => num_nodes = value.vals[0].to_str().unwrap(),
            None => num_nodes = "0",
        }

        match num_nodes.parse::<i32>() {
            Ok(nodes) => {
                if nodes <= 0 {
                    panic!("please provide a number of nodes bigger than 0")
                }

                let num_threads = if nodes < 4 { nodes } else { 4 };

                match utils::check_dependencies() {
                    Ok(_) => (),
                    Err(..) => {
                        panic!(
                            "make sure you have docker, docker-machine and virtualbox installed"
                        );
                    }
                }

                let float_split = nodes as f64 / num_threads as f64;

                let split_val = if (float_split % 1.) > 0.5 {
                    float_split.ceil() as i32
                } else {
                    float_split.floor() as i32
                };

                let mut split_vec_values = vec![0 as i32; num_threads as usize];

                for i in 0..(num_threads - 1) {
                    split_vec_values[i as usize] = split_val;
                }

                let last_item = nodes - (split_val * (num_threads - 1));

                split_vec_values[(num_threads - 1) as usize] =
                    if last_item > 0 { last_item } else { 1 };

                let value_vec: Arc<Vec<i32>> = Arc::new(split_vec_values);

                for position in 0..num_threads {
                    let cloned_vec = Arc::clone(&value_vec);
                    crossbeam::thread::scope(move |_| {
                        let pos: i32 = position.clone();
                        let vec_value: i32 = cloned_vec[pos as usize];
                        let start_pos = (cloned_vec[0] * pos) + 1;

                        for worker in start_pos..(start_pos + vec_value) {
                            smol::block_on(async {
                                match smol::process::Command::new("sh")
                                    .args(&[
                                        "-c",
                                        format!(
                                            "docker-machine create -d virtualbox worker{}",
                                            &worker
                                        )
                                        .as_str(),
                                    ])
                                    .output()
                                    .await
                                {
                                    Ok(_) => {
                                        println!("starting worker{}...", worker);
                                    }
                                    Err(why) => panic!("not able to execute command: {}", &why),
                                }
                            });
                        }
                    })
                    .unwrap();
                }
            }

            Err(_) => panic!("The value provided is not an integer"),
        }
    }
}

impl CommandRunner for CreateCommand {
    fn run(&self, opts: &clap::ArgMatches<'_>) -> Result<()> {
        match opts.subcommand() {
            ("worker", Some(args)) => self.run_worker(args),
            _ => panic!("no subcommands were provided or subcommand not found!"),
        }

        return Ok(());
    }
}
