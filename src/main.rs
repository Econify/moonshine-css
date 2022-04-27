
use serde::Deserialize;

use std::collections::BTreeMap;
use std::fs::File;
use std::io::BufReader;
use std::fs;

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
#[serde(tag = "method")]
pub enum Instruction {
    FromVariableMap(FromVariableMap),
    SingleRuleFromVariableGroup(FromVariableMap),
    ManyRulesFromVariableMatrix(ManyRulesFromVariableMatrix),
    // ManyRulesFromVariableMap(ManyRulesFromVariableMatrix),
}



#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct FromVariableMap {
    description: String,
    map_name: String,
    selector: String,
    declarations: BTreeMap<String, String>
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ManyRulesFromVariableMap {
    description: String,
    map_name: String,
    selector: String,
    declarations: BTreeMap<String, String>
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ManyRulesFromVariableMatrix {
    description: String,
    map_name_a: String,
    map_name_b: String,
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
    declarations: BTreeMap<String, String>,
}

fn err_msg_for_missing_map(description: &str, map_name: &str) -> String {
    format!(
        "{}: There is no variable map named {}",
        description,
        map_name
    )
}

/// Derive a single `CSSRule` using `FromVariableMap`
fn single_rule_from_variable_map(config: &Config, inst: &FromVariableMap) -> Vec<CSSRule> {
    let variable_map = config.variable_maps
        .get(&inst.map_name)
        .expect(&err_msg_for_missing_map(&inst.description, &inst.map_name));

    let selector = inst.selector.clone();
    let mut declarations = BTreeMap::new();

    for (var_key, var_val) in variable_map {
        let inject_variables = |s: &String| s
            .replace("{{ KEY }}", var_key)
            .replace("{{ VAL }}", var_val);

        for (property, value) in &inst.declarations {
            declarations.insert(
                inject_variables(&property),
                inject_variables(&value),
            );
        }
    }

    vec![
        CSSRule { selector, declarations }
    ]
}

/// Derive many `CSSRule`s using `ManyRulesFromVariableMatrix`
fn many_rules_from_variable_matrix(config: &Config, inst: &ManyRulesFromVariableMatrix) -> Vec<CSSRule> {
    let mut rules = vec![];

    let variable_map_a = config.variable_maps
        .get(&inst.map_name_a)
        .expect(&err_msg_for_missing_map(&inst.description, &inst.map_name_a));

    let variable_map_b = config.variable_maps
        .get(&inst.map_name_b)
        .expect(&err_msg_for_missing_map(&inst.description, &inst.map_name_b));

    for (key_a, val_a) in variable_map_a {
        for (key_b, val_b) in variable_map_b {
            let inject_variables = |s: &String| s
                .replace("{{ KEY_A }}", key_a)
                .replace("{{ KEY_B }}", key_b)
                .replace("{{ VAL_A }}", val_a)
                .replace("{{ VAL_B }}", val_b);
            
            rules.push(CSSRule {
                selector: inject_variables(&inst.css_selector),
                declarations: BTreeMap::from([(
                    inject_variables(&inst.css_property),
                    inject_variables(&inst.css_value)
                )]),
            });
        }
    }

    rules
}

/// Derive `CSSRule`s using `FromVariableMap`
fn from_variable_map(config: &Config, inst: &FromVariableMap) -> Vec<CSSRule> {
    let variable_map = config.variable_maps
        .get(&inst.map_name)
        .expect(&err_msg_for_missing_map(&inst.description, &inst.map_name));
    
    variable_map.iter().map(|(var_key, var_val)| {
        let inject_variables = |s: &String| s
            .replace("{{ KEY }}", var_key)
            .replace("{{ VAL }}", var_val);
        
        CSSRule {
            selector: inject_variables(&inst.selector),
            declarations: inst.declarations.iter().map(|(property, value)| {
                (
                    inject_variables(&property),
                    inject_variables(&value),
                )
            }).collect()
        }
    }).collect()
}

fn generate_rules(config: Config) -> Vec<CSSRule> {
    let mut rules = Vec::new();

    for instruction in &config.instructions {
        match instruction {
            Instruction::FromVariableMap(inst) => {
                rules.extend(from_variable_map(&config, &inst))
            }
            Instruction::SingleRuleFromVariableGroup(inst) => {
                rules.extend(single_rule_from_variable_map(&config, &inst))
            }
            Instruction::ManyRulesFromVariableMatrix(inst) => {
                rules.extend(many_rules_from_variable_matrix(&config, &inst))
            }
            // Instruction::ManyRulesFromVariableMap(inst) => {
            //     rules.extend(many_rules_from_variable_matrix(&config, &inst))
            // }
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
    fs::write("./build.css", css).expect("Unable to write file")
}
