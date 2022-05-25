use std::fs;
use std::path::Path;

pub fn write_file_creating_dirs(path: &str, contents: &str) {
    let path = Path::new(path);
    let parent_dir = path.clone().parent().unwrap();
    fs::create_dir_all(parent_dir).unwrap();
    fs::write(path.clone(), contents).unwrap();
}
