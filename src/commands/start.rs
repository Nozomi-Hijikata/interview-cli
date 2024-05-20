use structopt::StructOpt;

#[derive(StructOpt)]
pub struct Start {
    #[structopt(short, long, help = "Path to the file")]
    template: String,
}

impl Start {
    pub fn run(&self) {
        println!("start the process");
        println!("template: {}", self.template);
    }
}
