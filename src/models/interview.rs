use log::*;
use markdown::{to_mdast, ParseOptions};
use std::io;
use std::path::Path;
use crate::parser::parse_answers;
use crate::utils::file_handler::read_file;



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

        let content = read_file(file_path)?;

        debug!("Content: {}", content);

        let mdast = to_mdast(&content, &ParseOptions::default()).unwrap();

        debug!("MDAST: {:?}", mdast);

        let answers = parse_answers(&mdast);

        debug!("Answers: {:?}", answers);

        self.calculate_score(&answers);

        info!("Total score for {}: {}", self.candidate_name, self.total_score);

        Ok(())
    }

    fn calculate_score(&mut self, answers: &[(&str, u32)]) {
        self.total_score = answers.iter().map(|(_, score)| score).sum();
    }
}
