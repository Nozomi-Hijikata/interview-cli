mod assets;
mod commands;
mod config;
mod models;
mod parser;
mod utils;

use commands::init::Init;
use commands::start::Start;
use commands::sync::Sync;
use structopt::StructOpt;

#[derive(StructOpt)]
#[structopt(name = "interview-cli", about = "A CLI tool for interview evaluation")]
enum Cli {
    /// Initialize the interview environment
    Init(Init),
    /// Start the interview
    Start(Start),
    /// Sync the interview data
    Sync(Sync),
}

fn main() {
    config::init();
    let args = Cli::from_args();
    match args {
        Cli::Init(init) => init.run(),
        Cli::Start(start) => start.run(),
        Cli::Sync(sync) => sync.run(),
    }
}
