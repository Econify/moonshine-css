
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

#[derive(Default)]
struct Rule {
    selector: String,
    declarations: HashMap<String, String>,
}

fn create_css_from_config(config: Config) {
    let mut rules: HashMap<String, String> = HashMap::new();

    // Text Color
    const TEXT_COLOR_PREFIX: &'static str = "";

    for (name, value)  in config.color {
        let mut rule = Rule::default();
        rule.selector = name;
        rule.declarations.insert("color".to_string(), value);
    }

    // Background Color
    // Display
    // Flex Related
    // Grid Related
}

fn main() {
    let path = std::env::args().nth(1)
        .unwrap_or("config.json".to_string());

    let file = File::open(path).unwrap();
    let reader = BufReader::new(file);
    let config: Config = serde_json::from_reader(reader).unwrap();
}
