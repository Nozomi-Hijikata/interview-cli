use chrono::Local;
use std::path::Path;

use log::*;
use structopt::StructOpt;

use crate::utils::file_handler::{copy_file, create_directory};

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
        debug!("template: {}", self.template);
        debug!("name: {}", self.name);

        // let current_date = Local::now().format("%Y%m%d").to_string();
        // let candidate_dir_name = format!("{}_{}", current_date, self.name);
        let candidate_dir_name = &self.name;
        let candidate_dir = Path::new("interviews").join(&candidate_dir_name);

        create_directory(&candidate_dir);

        let template_path = Path::new("templates").join(&self.template);
        debug!("template_path: {}", template_path.display());
        let destination_path = candidate_dir.join("interviews.md");

        copy_file(&template_path, &destination_path);
    }
}
