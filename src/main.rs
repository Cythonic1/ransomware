use std::{fs, path::Path};
mod file_handler;
fn main() {
    let path = Path::new("./");
    let files = file_handler::read_directory::list_directories(path);
    for file in files {
        println!("file : {}", file.path().display());
    }
}
