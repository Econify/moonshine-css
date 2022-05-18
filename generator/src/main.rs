
mod lib;

use lib::{Transformations, TokenGroups, Intermediate};
use std::io::BufReader;
use std::fs;

fn main() {
    let path_to_tokens = std::env::args().nth(1)
        .unwrap_or("atomic-styles.design-tokens.json".to_string());
    let tokens_file = fs::File::open(path_to_tokens).unwrap();
    let reader = BufReader::new(tokens_file);
    let inputs: TokenGroups = serde_json::from_reader(reader).unwrap();


    let path_to_transformations = std::env::args().nth(2)
        .unwrap_or("atomic-styles.transformations.json".to_string());
    let inputs_file = fs::File::open(path_to_transformations).unwrap();
    let reader = BufReader::new(inputs_file);
    let transformations: Transformations = serde_json::from_reader(reader).unwrap();


    let intermediate = Intermediate::build(inputs, transformations);
    let css = intermediate.stringify();
    let intermediate = serde_json::to_string_pretty(&intermediate).unwrap();
    fs::write("./build.css", css).expect("Unable to write file");
    fs::write("./intermediate.json", intermediate).expect("Unable to write file");
}
