use crate::models::interview::Interview;
use log::*;
use std::path::Path;
use structopt::StructOpt;

#[derive(StructOpt)]
pub struct Sync {
    #[structopt(short = "n", long, help = "Name of the candidate")]
    candidate_name: String,
}

impl Sync {
    pub fn run(&self) {
        info!("Syncing the interview scores");
        let interviews_dir = Path::new("interviews");
        let candidate_dir_file = interviews_dir
            .join(&self.candidate_name)
            .join("interviews.md");

        let mut interview = Interview::new(&self.candidate_name);
        if let Err(e) = interview.sync(&candidate_dir_file) {
            error!("Error syncing the interview scores: {}", e);
        }else {
            info!("Interview scores synced successfully");
        }
    }
}
