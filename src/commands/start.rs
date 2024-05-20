use structopt::StructOpt;

#[derive(StructOpt)]
pub struct Start {
    #[structopt(short="t", long, help = "Path to the file")]
    template: String,
    #[structopt(short="n", long, help = "Name of the interviewee")]
    name: String,
}

impl Start {
    pub fn run(&self) {
        println!("start the process");
        println!("template: {}", self.template);
        println!("name: {}", self.name);
    }
}
