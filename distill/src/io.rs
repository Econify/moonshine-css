use std::fs;
use std::path::Path;

pub fn write_file_creating_dirs(path: &str, contents: &str) -> Result<(), &'static str> {
    let path = Path::new(path);

    let parent_dir = match path.clone().parent() {
        None => return Err("Unable to find parent directory"),
        Some(dir) => dir,
    };

    match fs::create_dir_all(parent_dir) {
        Err(_why) => return Err("Unable to create parent directory"),
        Ok(_) => (),
    };

    match fs::write(path.clone(), contents) {
        Err(_why) => Err("Unable to write contents to file"),
        Ok(_) => Ok(()),
    }
}
