
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
struct Rule {
    selector: String,
    declarations: Vec<(String, String)>,
}


fn generate_rules(config: Config) -> Vec<Rule> {
    let mut rules = Vec::new();

    for instruction in config.instructions {
        match instruction {
            Instruction::FromVariableMap(inst) => {

                let err_msg_for_missing_map = format!(
                    "{}: There is no variable map named {}",
                    inst.description,
                    inst.map_name
                );

                let variable_map = config.variable_maps
                    .get(&inst.map_name)
                    .expect(&err_msg_for_missing_map);

                
                for (key, val) in variable_map {
                    let mut rule = Rule::default();

                    let inject_variables = |string: &String| {
                        string
                            .replace("{{ VAR_KEY }}", key)
                            .replace("{{ VAR_VAL }}", val)
                    };

                    rule.selector = inject_variables(&inst.css_selector);

                    rule.declarations = vec![(
                        inject_variables(&inst.css_property),
                        inject_variables(&inst.css_value),
                    )];

                    rules.push(rule);
                }
            }
        }
    }

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
