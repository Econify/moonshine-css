
use serde_yaml as yaml;
use serde::Deserialize;
use regex::Regex;

use super::lib::{
    Transformation,
    Transformations,
    ManyRulesFromTokenGroup,
    CSSRule,
};

use std::collections::{HashMap, BTreeMap};


const _SRC: &str = r#"

bg-[$colors.key]:
  background-color: var(--$colors.key)

bg-[$colors.key]:
  background-color: var(--$colors.key)

mt-[$sizes.key]:
  margin-top: $sizes.value

m-[$sizes.key]:
  margin: $sizes.value

"#;

type AtomName = String;
type TokenGroupName = String;
type CSSProperty = String;
type CSSValue = String;

pub type SugarRuleSet = HashMap<AtomName, SugarBlock>;
pub type SugarBlock = HashMap<CSSProperty, CSSValue>;

pub fn transformations_from_sugar_rules(ruleset: &SugarRuleSet) -> Transformations {
    let mut list = Vec::new();

    for (atom_name_template, block) in ruleset {
        match detect_token_loop(atom_name_template, block) {
            Some(config) => list.push(Transformation::ManyRulesFromTokenGroup(config)),
            None => (),
        }
    }

    list 
}

fn detect_token_loop(atom_name_template: &str, block: &SugarBlock) -> Option<ManyRulesFromTokenGroup> {
    let re = Regex::new(r"(?P<before>.*)\[\$(?P<token_group_name>.*)(?P<key_or_value>(\.key)|(\.value))\](?P<after>.*)").unwrap();

    if false == re.is_match(&atom_name_template) {
        return None
    }

    let token_group_name = re.replace(atom_name_template, "$token_group_name").to_string();

    let key_list_replacer = format!("[${}.key]", token_group_name);
    let value_list_replacer = format!("[${}.value]", token_group_name);

    let key_replacer = format!("${}.key", token_group_name);
    let value_replacer = format!("${}.value", token_group_name);

    let atom_name = atom_name_template
        .replace(&key_list_replacer, "{{ KEY }}")
        .replace(&value_list_replacer, "{{ VAL }}");

    let mut rule = CSSRule {
        selector: get_atom_selector(&atom_name),
        declarations: BTreeMap::new(),
    };

    for (property_template, value_template) in block {

        let property = property_template
            .replace(&key_replacer, "{{ KEY }}")
            .replace(&value_replacer, "{{ VAL }}");

        let value = value_template
            .replace(&key_replacer, "{{ KEY }}")
            .replace(&value_replacer, "{{ VAL }}");

        rule.declarations.insert(property, value);
    }

    Some(ManyRulesFromTokenGroup {
        id: atom_name.to_string(),
        description: "".to_string(),
        token_group_name,
        rules: vec![rule],
    })
}

fn get_atom_selector(atom_name: &str) -> String {
    format!(".{}", atom_name)
}

#[test]
fn detect_loop() {
    let ruleset: SugarRuleSet = yaml::from_str(_SRC).unwrap();

    let transforms = transformations_from_sugar_rules(&ruleset);
    println!("{}", yaml::to_string(&transforms).unwrap());
}
