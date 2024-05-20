use structopt::StructOpt;

#[derive(StructOpt)]
pub struct Sync {}

impl Sync {
    pub fn run(&self) {
        println!("start the synchronization process");
    }
}
