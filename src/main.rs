
use serde::Deserialize;

use std::collections::HashMap;
use std::fs::File;
use std::io::BufReader;

#[derive(Deserialize, Debug)]
struct Config {
    color: HashMap<String, String>,
}

fn main() {
    let path = "config.json";
    let file = File::open(path).unwrap();
    let reader = BufReader::new(file);
    let config: Config = serde_json::from_reader(reader).unwrap();
    println!("{:#?}", config);
}
