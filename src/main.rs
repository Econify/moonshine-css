
use serde::Deserialize;

use std::collections::BTreeMap;
use std::fs::File;
use std::io::BufReader;
use std::fs;

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
#[serde(tag = "method")]
pub enum Instruction {
    SingleRuleFromVariableGroup(FromVariableGroup),
    ManyRulesFromVariableGroup(ManyRulesFromVariableGroup),
}



#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct FromVariableGroup {
    id: String,
    description: String,
    variable_group: String,
    selector: String,
    declarations: BTreeMap<String, String>
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ManyRulesFromVariableGroup {
    id: String,
    description: String,
    variable_group: String,
    rules: Vec<CSSRule>,
}


pub type VariableGroup = BTreeMap<String, String>;

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
struct Config {
    variable_groups: BTreeMap<String, VariableGroup>,
    instructions: Vec<Instruction>,
}

#[derive(Deserialize, Debug, Default)]
struct CSSRule {
    selector: String,
    declarations: BTreeMap<String, String>,
}

fn err_msg_for_missing_map(description: &str, variable_group: &str) -> String {
    format!(
        "{}: There is no variable map named {}",
        description,
        variable_group
    )
}

/// Derive a single `CSSRule` using `FromVariableGroup`
fn many_rules_from_variable_group(config: &Config, inst: &ManyRulesFromVariableGroup) -> Vec<CSSRule> {
    let variable_group = config.variable_groups
        .get(&inst.variable_group)
        .expect(&err_msg_for_missing_map(&inst.description, &inst.variable_group));

    let mut rules = vec![];

    for rule in &inst.rules {
        for (var_key, var_val) in variable_group {
            let inject_variables = |s: &String| s
                .replace("{{ KEY }}", var_key)
                .replace("{{ VAL }}", var_val);
            
            rules.push(CSSRule {
                selector: inject_variables(&rule.selector),
                declarations: rule.declarations.iter().map(|(property, value)| {
                    (
                        inject_variables(&property),
                        inject_variables(&value),
                    )
                }).collect()
            })
        }

    }

    rules
}

/// Derive a single `CSSRule` using `FromVariableGroup`
fn single_rule_from_variable_group(config: &Config, inst: &FromVariableGroup) -> Vec<CSSRule> {
    let variable_group = config.variable_groups
        .get(&inst.variable_group)
        .expect(&err_msg_for_missing_map(&inst.description, &inst.variable_group));

    let selector = inst.selector.clone();
    let mut declarations = BTreeMap::new();

    for (var_key, var_val) in variable_group {
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

fn generate_rules(config: Config) -> Vec<CSSRule> {
    let mut rules_by_id = BTreeMap::new();

    for instruction in &config.instructions {
        match instruction {
            Instruction::SingleRuleFromVariableGroup(inst) => {
                rules_by_id.insert(inst.id.clone(), single_rule_from_variable_group(&config, &inst));
            }
            Instruction::ManyRulesFromVariableGroup(inst) => {
                rules_by_id.insert(inst.id.clone(), many_rules_from_variable_group(&config, &inst));
            }
        }
    }

    let mut all_rules = vec![];

    for (_id, rules) in rules_by_id {
        all_rules.extend(rules);
    }

    all_rules
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
