
use std::process::exit;
use std::io::{ErrorKind, Error as IOError};


const ERR_PREFIX: &str = "❗️";

pub struct Exit;

impl Exit {
    pub fn with_message<T>(message: &str) -> T {
        println!("{}️ {}", ERR_PREFIX, message);
        exit(1);
    }  
}


pub fn describe_rc_file_open_error(err: IOError, filename: &str) -> String {
    match err.kind() {
        ErrorKind::NotFound => format!("Cannot find RC file: `{}`", filename),
        _any_other_kind => format!("Failed to open `{}`.", filename),
    }
}