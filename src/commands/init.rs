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

        if templates_dir.exists() {
            fs::remove_dir_all(templates_dir).unwrap();
        }
        if interviews_dir.exists() {
            fs::remove_dir_all(interviews_dir).unwrap();
        }

        assert!(!templates_dir.exists());
        assert!(!interviews_dir.exists());

        // Init構造体のインスタンスを作成してrunメソッドを実行
        let init = Init {};
        init.run();

        assert!(templates_dir.exists());
        assert!(interviews_dir.exists());

        fs::remove_dir_all(templates_dir).unwrap();
        fs::remove_dir_all(interviews_dir).unwrap();
    }
}
