
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
    AddClassModifier(AddClassModifier),
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct AddClassModifier {
    id: String,
    description: String,
    affected_ids: Vec<String>,
    class_prefix: String,
    represents_pseudo_class: String,
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

#[derive(Deserialize, Debug, Default, Clone)]
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

fn err_msg_for_missing_instruction(description: &str, id: &str) -> String {
    format!(
        "{}: There is no instruction named {}",
        description,
        id
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

/// Derive a new `CSSRule` for each existing one that was created
/// by an instruction with an ID in `affected_ids`
fn add_class_modifier(
    inst: &AddClassModifier,
    rules_by_id: &BTreeMap<String, Vec<CSSRule>>
) -> Vec<CSSRule> {
    let mut new_rules: Vec<CSSRule> = vec![];

    for id in &inst.affected_ids {
        let rules = rules_by_id.get(&id.clone())
            .expect(&err_msg_for_missing_instruction(&inst.description, &id));

        for rule in rules.iter() {

            // Skipping rules that don't target classes
            if !rule.selector.starts_with(".") { continue }

            let with_prefix = rule.selector.replacen(".", &inst.class_prefix, 1);
            let selector = format!(".{}:{}", with_prefix, inst.represents_pseudo_class);

            new_rules.push(CSSRule {
                selector,
                ..rule.clone()
            });
        }
    }

    new_rules
}

fn generate_rules(config: Config) -> Vec<CSSRule> {
    let mut rules_by_id: BTreeMap<String, Vec<CSSRule>> = BTreeMap::new();

    for instruction in &config.instructions {
        match instruction {
            Instruction::SingleRuleFromVariableGroup(inst) => {
                rules_by_id.insert(inst.id.clone(), single_rule_from_variable_group(&config, &inst));
            }
            Instruction::ManyRulesFromVariableGroup(inst) => {
                rules_by_id.insert(inst.id.clone(), many_rules_from_variable_group(&config, &inst));
            }
            Instruction::AddClassModifier(inst) => {
                rules_by_id.insert(inst.id.clone(), add_class_modifier(&inst, &rules_by_id));
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
