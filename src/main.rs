
use serde::Deserialize;

use std::collections::BTreeMap;
use std::fs::File;
use std::io::BufReader;

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
#[serde(tag = "method")]
pub enum Instruction {
    FromVariableMap(FromVariableMap),
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct FromVariableMap {
    description: String,
    map_name: String,
    css_selector: String,
    css_property: String,
    css_value: String,
}


pub type VariableMap = BTreeMap<String, String>;

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
struct Config {
    variable_maps: BTreeMap<String, VariableMap>,
    instructions: Vec<Instruction>,
}

#[derive(Default)]
struct CSSRule {
    selector: String,
    declarations: Vec<(String, String)>,
}

/// Derive `CSSRule`s using `FromVariableMap`
fn from_variable_map(config: &Config, inst: &FromVariableMap) -> Vec<CSSRule> {
    let mut rules = vec![];

    let err_msg_for_missing_map = format!(
        "{}: There is no variable map named {}",
        inst.description,
        inst.map_name
    );

    let variable_map = config.variable_maps
        .get(&inst.map_name)
        .expect(&err_msg_for_missing_map);

    for (key, val) in variable_map {
        let inject_variables = |string: &String| {
            string
                .replace("{{ VAR_KEY }}", key)
                .replace("{{ VAR_VAL }}", val)
        };

        rules.push(CSSRule {
            selector: inject_variables(&inst.css_selector),
            declarations: vec![(
                inject_variables(&inst.css_property),
                inject_variables(&inst.css_value),
            )],
        });
    }

    rules
}

fn global_variable_rule(config: &Config) -> CSSRule {
    let mut rule = CSSRule::default();
    rule.selector = ":root".to_string();

    for (_map_name, map) in &config.variable_maps {
        for (key, val) in map {
            rule.declarations.push((
                format!("--{}", key),
                format!("{}", val),
            ))
        }
    }

    rule
}

fn generate_rules(config: Config) -> Vec<CSSRule> {
    let mut rules = Vec::new();

    rules.push(global_variable_rule(&config));

    for instruction in &config.instructions {
        match instruction {
            Instruction::FromVariableMap(inst) => {
                rules.extend(from_variable_map(&config, &inst))
            }
        }
    }

    rules
}

fn stringify_rules(rules: Vec<CSSRule>) -> String {
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
