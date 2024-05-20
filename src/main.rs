mod commands;
mod models;
mod utils;

use structopt::StructOpt;
use commands::start::Start;

#[derive(StructOpt)]
#[structopt(name = "interview-cli", about = "A CLI tool for interview evaluation")]
enum Cli {
    Start(Start),
}

fn main() {
    let args = Cli::from_args();
    match args {
        Cli::Start(start) => start.run(),
    }
}
