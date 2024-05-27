use crate::assets::{OUTPUT_CONTENT, TEMPLATE_CONTENT};
use crate::utils::file_handler::{create_directory, write_file};
use log::*;
use std::path::Path;
use structopt::StructOpt;

#[derive(StructOpt)]
pub struct Init {}

impl Init {
    pub fn run(&self) {
        info!("Initializing the environment");

        let templates_dir = Path::new("templates");
        let interviews_dir = Path::new("interviews");

        create_directory(templates_dir);
        create_directory(interviews_dir);

        let destination_template_file = templates_dir.join("example.md");

        write_file(&destination_template_file, TEMPLATE_CONTENT);

        let destination_output_file = templates_dir.join("output.md");
        write_file(&destination_output_file, OUTPUT_CONTENT);

        info!("Initialization complete");
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use std::path::Path;

    #[test]
    fn test_run_creates_directories() {
        let templates_dir = Path::new("templates");
        let interviews_dir = Path::new("interviews");
        let template_file = templates_dir.join("example.md");

        if template_file.exists() {
            fs::remove_file(&template_file).unwrap();
        }
        if templates_dir.exists() {
            fs::remove_dir_all(templates_dir).unwrap();
        }
        if interviews_dir.exists() {
            fs::remove_dir_all(interviews_dir).unwrap();
        }

        assert!(!templates_dir.exists());
        assert!(!interviews_dir.exists());

        let init = Init {};
        init.run();

        assert!(templates_dir.exists());
        assert!(interviews_dir.exists());
        assert!(template_file.exists());

        fs::remove_dir_all(templates_dir).unwrap();
        fs::remove_dir_all(interviews_dir).unwrap();
    }
}
