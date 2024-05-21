use std::fs;
use std::path::Path;
use structopt::StructOpt;

#[derive(StructOpt)]
pub struct Init {}

impl Init {
    pub fn run(&self) {
        println!("Initializing the environment");

        let templates_dir = Path::new("templates");
        let interviews_dir = Path::new("interviews");

        if !templates_dir.exists() {
            match fs::create_dir(templates_dir) {
                Ok(_) => println!("Created 'templates' directory"),
                Err(e) => eprintln!("Failed to create 'templates' directory: {}", e),
            }
        } else {
            println!("'templates' directory already exists");
        }

        if !interviews_dir.exists() {
            match fs::create_dir(interviews_dir) {
                Ok(_) => println!("Created 'interviews' directory"),
                Err(e) => eprintln!("Failed to create 'interviews' directory: {}", e),
            }
        } else {
            println!("'interviews' directory already exists");
        }

        let template_file = templates_dir.join("example.md");
        if !template_file.exists() {
            match fs::write(
                &template_file,
                "# Example Template\n\n- Question 1\n- Question 2",
            ) {
                Ok(_) => println!("Created 'example.md' template"),
                Err(e) => eprintln!("Failed to create 'example.md' template: {}", e),
            }
        }

        println!("Initialization complete");
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
