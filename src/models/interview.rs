use log::*;
use markdown::{to_mdast, ParseOptions};
use std::fs::{self, File};
use std::io::{self, Read, Write};
use std::path::Path;
use crate::parser::parse_answers;



pub struct Interview {
    pub total_score: u32,
    pub candidate_name: String,
}

impl Interview {
    pub fn new(candidate_name: &str) -> Self {
        Self {
            total_score: 0,
            candidate_name: candidate_name.to_string(),
        }
    }

    pub fn sync(&mut self, file_path: &Path) -> io::Result<()> {
        debug!("Syncing interview scores for {}", self.candidate_name);
        debug!("File path: {:?}", file_path);

        let content = self.read_file(file_path)?;

        debug!("Content: {}", content);

        let mdast = to_mdast(&content, &ParseOptions::default()).unwrap();

        debug!("MDAST: {:?}", mdast);

        let answers = parse_answers(&mdast);

        debug!("Answers: {:?}", answers);
        // self.calculate_score(&answers);
        // self.write_score(file_path, &self.total_score.to_string());
        Ok(())
    }

    // fn calculate_score(&mut self, answers: &[(&str, u32)]) {
    //     self.total_score = answers.iter().map(|(_, score)| score).sum();
    // }
    //
    fn read_file(&self, file_path: &Path) -> io::Result<String> {
        let mut file = File::open(file_path)?;
        let mut content = String::new();
        file.read_to_string(&mut content)?;
        Ok(content)
    }
    //
    // fn write_score(&self, file_path: &Path, score: &str) -> io::Result<()> {
    //     let mut file = File::open(file_path)?;
    //     let mut content = String::new();
    //     file.read_to_string(&mut content)?;
    //     let new_content = content.replace("{{score}}", score);
    //     let mut file = File::create(file_path)?;
    //     file.write_all(new_content.as_bytes())?;
    //     Ok(())
    // }
    //
}
