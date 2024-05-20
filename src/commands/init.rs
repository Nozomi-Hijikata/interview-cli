use structopt::StructOpt;

#[derive(StructOpt)]
pub struct Init {}

impl Init {
    pub fn run(&self) {
        println!("initialize the environment");
    }
}
