use crate::utils::file_handler::{copy_file, create_directory};
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

        create_directory(&templates_dir);
        create_directory(&interviews_dir);

        let source_template_file = Path::new("src/assets").join("example.md");
        let destination_template_file = templates_dir.join("example.md");

        copy_file(&source_template_file, &destination_template_file);

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
