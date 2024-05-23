use log::*;
use std::fs;
use std::path::Path;

pub fn create_directory(path: &Path) {
    if !path.exists() {
        match fs::create_dir(path) {
            Ok(_) => info!("Created directory: {}", path.display()),
            Err(e) => error!("Failed to create directory: {}", e),
        }
    } else {
        info!("Directory already exists: {}", path.display());
    }
}

pub fn copy_file(source: &Path, destination: &Path) {
    if !destination.exists() {
        match fs::copy(source, destination) {
            Ok(_) => info!(
                "Copied file: {} to {}",
                source.display(),
                destination.display()
            ),
            Err(e) => error!("Failed to copy file: {}", e),
        }
    } else {
        info!("File already exists: {}", destination.display());
    }
}
