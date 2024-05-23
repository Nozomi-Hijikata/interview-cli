use regex::Regex;
use std::fs::{self, File};
use std::io::{self, Read, Write};
use std::path::Path;
use log::*;

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
        // let answers = self.parse_answers(&content);
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
    // fn parse_answers(&self, content: &String) -> Vec<(&str, u32)> {
    //     let mut answers = Vec::new();
    //     let question_re =
    //         Regex::new(r"(?m)^# 質問\d+\n(.*?)\n## 研修生の回答\n(.*?)\n## 点数\n(\d+)").unwrap();
    //
    //     for cap in question_re.captures_iter(content) {
    //         let question = cap.get(1).map_or("", |m| m.as_str()).trim();
    //         let score: u32 = cap.get(3).map_or("0", |m| m.as_str()).parse().unwrap_or(0);
    //         answers.push((question, score));
    //     }
    //
    //     answers
    // }
}
