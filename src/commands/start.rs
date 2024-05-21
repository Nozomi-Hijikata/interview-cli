use chrono::Local;
use std::fs;
use std::path::Path;
use structopt::StructOpt;

use log::*;

#[derive(StructOpt)]
pub struct Start {
    #[structopt(short = "t", long, help = "Path to the file")]
    template: String,
    #[structopt(short = "n", long, help = "Name of the interviewee")]
    name: String,
}

impl Start {
    pub fn run(&self) {
        info!("Creating the candidate's directory...");
        info!("template: {}", self.template);
        info!("name: {}", self.name);

        let current_date = Local::now().format("%Y%m%d").to_string();
        let candidate_dir_name = format!("{}_{}", current_date, self.name);
        let candidate_dir = Path::new("interviews").join(&candidate_dir_name);
        if !candidate_dir.exists() {
            match fs::create_dir(&candidate_dir) {
                Ok(_) => info!("Candidate's directory was created"),
                Err(e) => error!(
                    "Error creating directory: {} Probably you should initialize the project first",
                    e
                ),
            }
        }
    }
}
