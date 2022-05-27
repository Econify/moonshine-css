use std::fs;
use std::path::Path;

const RC_FILE_SRC: &str = r#"
{
  "designTokens": [
    "./advanced/atomic-styles.design-tokens.json"
  ],
  "transformations": [
    "./advanced/atomic-styles.transformations.json"
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
