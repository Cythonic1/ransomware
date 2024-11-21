use std::fs::{self, DirEntry};
use std::path::Path;

pub fn list_directories(dir: &Path) -> Vec<DirEntry> {
    let mut files = Vec::new();

    match fs::read_dir(dir) {
        Ok(entries) => {
            for entry in entries {
                match entry {
                    Ok(entry) => files.push(entry), // Collect `DirEntry` into the vector
                    Err(e) => println!("Failed to read entry: {e}"),
                }
            }
        }
        Err(e) => println!("Failed to read directory: {e}"),
    }

    files
}

pub fn is_file_or_dir(entries: Vec<DirEntry>) {
    for entry in entries {
        match entry.metadata() {
            Ok(metadata) => {
                if metadata.is_file() {
                    println!("The path: {} is a file", entry.path().display());
                } else if metadata.is_dir() {
                    println!("The path: {} is a directory", entry.path().display());
                }
            }
            Err(err) => {
                println!("Failed to read metadata: {err}");
            }
        }
    }
}
