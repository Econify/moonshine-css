use super::io;
use super::{Exit};

use io::write_file_creating_dirs;
use std::fs;
use std::path::Path;

const RC_FILE_SRC: &str = r#"{
  "options": {
    "atomStyle": "classAttribute"
  },
  "designTokens": [
    "./design-tokens.yml"
  ],
  "templates": [
    "./templates/example.yml"
  ],
  "output": {
    "cssVariables": "./dist/variables.css",
    "cssAtoms": "./dist/atoms.css",
    "jsonAtoms": "./dist/atoms.json"
  }
}
"#;

const TOKENS_FILE_SRC: &str = r###"{
  "colors": {
    "blue": "#264b96",
    "green": "#006f3c",
    "red": "#bf212f"
  }
}
"###;

const EXAMPLE_TEMPLATE_SRC: &str = r###"# Background Colors
bg-[$colors.key]:
  background-colors: var(--$colors.key)
"###;

pub fn initialize_moonshinerc(path: &str) {
    if Path::new(path).exists() {
        Exit::with_message(
            &format!("RC File already exists: `{}`.", path)
        )
    }

    println!("Initializing `.moonshinerc`");

    fs::write(path, RC_FILE_SRC).unwrap_or(
        Exit::with_message(
            &format!("Failed to write file: {}.", path)
        )
    );

    fs::write("./design-tokens.yml", TOKENS_FILE_SRC).unwrap_or(
        Exit::with_message("Unable to write design-tokens.yml to current directory")
    );

    write_file_creating_dirs("./templates/example.yml", EXAMPLE_TEMPLATE_SRC).unwrap_or(
        Exit::with_message("Unable to write to ./templates/example.yml")
    );

    println!("\x1b[32mDone\x1b[0m - now run 'distill' to start using Moonshine CSS");
}
