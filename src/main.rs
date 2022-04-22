
use serde::Deserialize;

use std::collections::HashMap;
use std::fs::File;
use std::io::BufReader;

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
struct Config {
    color: HashMap<String, String>,
    font: HashMap<String, String>,
    gradient: HashMap<String, String>,
    font_size: Vec<f32>,
    line_height: Vec<f32>,
    opacity: Vec<u32>,
    border_radius: Vec<u32>,
    paragraph_spacing: Vec<u32>,
    letter_spacing: Vec<u32>,
    spacing: Vec<u32>,
}

fn main() {
    let path = "config.json";
    let file = File::open(path).unwrap();
    let reader = BufReader::new(file);
    let config: Config = serde_json::from_reader(reader).unwrap();
    println!("{:#?}", config);
}
