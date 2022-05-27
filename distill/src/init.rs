use std::fs;
use std::path::Path;

const RC_FILE_SRC: &str = r#"
{
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
    "css": "./dist/styles.css",
    "json": "./dist/styles.json"
  }
}
"#;

pub fn initialize_moonshinerc(path: &str) {
    if Path::new(path).exists() {
        println!("File \"{}\" already exists", path);
        return;
    }

    println!("Initializing .moonshinerc");

    fs::write(path, RC_FILE_SRC).expect("Unable to write .moonshinerc to current directory");

    println!("\x1b[32mDone\x1b[0m - now run 'distill' to start using Moonshine CSS");
}
