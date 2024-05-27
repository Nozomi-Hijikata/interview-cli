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

        let candidate_dir_name = &self.name;
        let candidate_dir = Path::new("interviews").join(candidate_dir_name);

        create_directory(&candidate_dir);

        let template_path = Path::new("templates").join(&self.template);
        debug!("template_path: {}", template_path.display());
        let destination_path = candidate_dir.join("interviews.md");

        copy_file(&template_path, &destination_path);

        let output_path = Path::new("templates").join("output.md");
        let destination_output_path = candidate_dir.join("output.md");
        debug!("output_path: {}", output_path.display());

        copy_file(&output_path, &destination_output_path);
    }
}
