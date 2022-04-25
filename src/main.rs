
use serde::Deserialize;

use std::collections::BTreeMap;
use std::fs::File;
use std::io::BufReader;


#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
struct Config {
    color: BTreeMap<String, String>,
    font: BTreeMap<String, String>,
    gradient: BTreeMap<String, String>,
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
    declarations: Vec<(String, String)>,
}

fn generate_rules(config: Config) -> Vec<Rule> {
    let mut rules = Vec::new();

    // Text Color
    const TEXT_COLOR_PREFIX: &'static str = "";

    for (name, value)  in config.color {
        let mut rule = Rule::default();
        rule.selector = format!(".{}", name);
        rule.declarations.push(("color".to_string(), value));
        rules.push(rule);
    }

    // Background Color
    // Display
    // Flex Related
    // Grid Related

    rules
}

fn stringify_rules(rules: Vec<Rule>) -> String {
    let mut css = String::new();

    for rule in rules {
        let inner = rule.declarations.iter()
            .map(|(k, v)| format!("{}:{};", k, v))
            .collect::<Vec<String>>()
            .join("");

        let line = format!("{}{{{}}}\n", rule.selector, inner);
        css.push_str(&line);
    }

    css
}

fn main() {
    let path = std::env::args().nth(1)
        .unwrap_or("config.json".to_string());

    let file = File::open(path).unwrap();
    let reader = BufReader::new(file);
    let config: Config = serde_json::from_reader(reader).unwrap();
    let rules = generate_rules(config);
    let css = stringify_rules(rules);
    println!("{}", css);
}
