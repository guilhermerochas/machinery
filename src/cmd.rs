use std::io::Result;

pub trait CommandRunner {
    fn run(&self, opts: &clap::ArgMatches) -> Result<()>;
}
