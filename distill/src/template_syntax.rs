use indexmap::IndexMap;
use regex::Regex;
use serde::{Deserialize, Serialize};

use super::transformation_syntax::{
    CSSRule, ManyRulesFromTokenGroup, NoTransformation, Transformation, Transformations,
};

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[serde(default)]
pub struct Options {
    pub non_atom_identifier: String,
    pub atom_style: AtomStyle,
    pub breakpoints: IndexMap<String, Breakpoint>,
}

#[derive(Default, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[serde(default)]
pub struct Breakpoint {
    min_width: Option<String>,
    max_width: Option<String>,
}

impl Default for Options {
    fn default() -> Self {
        Self {
            non_atom_identifier: "__non_atom__".to_string(),
            atom_style: AtomStyle::ClassAttribute,
            breakpoints: IndexMap::default(),
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum AtomStyle {
    ClassAttribute,
    DataAttribute,
}

type AtomName = String;
type CSSProperty = String;
type CSSValue = String;
type VariableMaps = IndexMap<String, IndexMap<String, String>>;

pub type CSSTemplate = IndexMap<AtomName, SugarBlock>;
pub type SugarBlock = IndexMap<CSSProperty, CSSValue>;

pub fn transformations_from_templates(ruleset: &CSSTemplate, options: &Options) -> Transformations {
    let mut list = Vec::new();

    let mut variable_maps: VariableMaps = IndexMap::new();

    for (atom_name_template, block) in ruleset {
        match detect_variable_map_declaration(atom_name_template, block, &mut variable_maps) {
            true => {
                continue;
            }
            false => (),
        };

        match detect_variable_map_loop(atom_name_template, block, &variable_maps, &options) {
            None => (),
            Some(config) => {
                list.push(Transformation::NoTransformation(config));
                continue;
            }
        }

        match detect_token_loop(atom_name_template, block, &options) {
            None => (),
            Some(config) => {
                list.push(Transformation::ManyRulesFromTokenGroup(config));
                continue;
            }
        }

        // Assuming no transformation is required

        let mut rule = CSSRule {
            selector: get_selector(atom_name_template, &options),
            declarations: IndexMap::new(),
        };

        for (property, value) in block {
            rule.declarations
                .insert(property.to_string(), value.to_string());
        }

        let config = NoTransformation {
            id: atom_name_template.to_string(),
            description: "".to_string(),
            at_rule_identifier: None,
            rules: vec![rule],
        };

        list.push(Transformation::NoTransformation(config));
    }

    list
}

fn detect_variable_map_loop(
    atom_name_template: &str,
    block: &SugarBlock,
    variable_maps: &VariableMaps,
    options: &Options,
) -> Option<NoTransformation> {
    let re = Regex::new(r"(?P<before>.*)\[\$(?P<variable_map_name>.*)(?P<key_or_value>(\.key)|(\.value))\](?P<after>.*)").unwrap();

    if false == re.is_match(&atom_name_template) {
        return None;
    }

    let variable_map_name = re
        .replace(atom_name_template, "$variable_map_name")
        .to_string();

    let variable_map = match variable_maps.get(&variable_map_name) {
        None => return None,
        Some(map) => map,
    };

    let mut config = NoTransformation {
        id: atom_name_template.to_string(),
        description: "".to_string(),
        at_rule_identifier: None,
        rules: Vec::new(),
    };

    let key_list_replacer = format!("[${}.key]", variable_map_name);
    let value_list_replacer = format!("[${}.value]", variable_map_name);

    for (key, value) in variable_map {
        let atom_name = atom_name_template
            .replace(&key_list_replacer, &key)
            .replace(&value_list_replacer, &value);

        let mut css_rule = CSSRule {
            selector: get_selector(&atom_name, &options),
            declarations: IndexMap::new(),
        };

        let key_replacer = format!("${}.key", variable_map_name);
        let value_replacer = format!("${}.value", variable_map_name);

        for (block_key, block_val) in block {
            let css_property = block_key
                .replace(&key_replacer, &key)
                .replace(&value_replacer, &value);

            let css_value = block_val
                .replace(&key_replacer, &key)
                .replace(&value_replacer, &value);

            css_rule.declarations.insert(css_property, css_value);
        }

        config.rules.push(css_rule);
    }

    Some(config)
}

fn detect_variable_map_declaration(
    atom_name_template: &str,
    block: &SugarBlock,
    variable_maps: &mut VariableMaps,
) -> bool {
    let re = Regex::new(r"^\$(?P<variable_map_name>\S+)$").unwrap();
    if false == re.is_match(&atom_name_template) {
        return false;
    }

    let variable_map_name = re
        .replace(atom_name_template, "$variable_map_name")
        .to_string();
    variable_maps.insert(variable_map_name, block.clone());
    true
}

fn detect_token_loop(
    atom_name_template: &str,
    block: &SugarBlock,
    options: &Options,
) -> Option<ManyRulesFromTokenGroup> {
    let re = Regex::new(r"(?P<before>.*)\[\$(?P<token_group_name>.*)(?P<key_or_value>(\.key)|(\.value))\](?P<after>.*)").unwrap();

    if false == re.is_match(&atom_name_template) {
        return None;
    }

    let token_group_name = re
        .replace(atom_name_template, "$token_group_name")
        .to_string();

    let key_list_replacer = format!("[${}.key]", token_group_name);
    let value_list_replacer = format!("[${}.value]", token_group_name);

    let key_replacer = format!("${}.key", token_group_name);
    let value_replacer = format!("${}.value", token_group_name);

    let atom_name = atom_name_template
        .replace(&key_list_replacer, "{{ KEY }}")
        .replace(&value_list_replacer, "{{ VAL }}");

    let mut rule = CSSRule {
        selector: get_selector(&atom_name, &options),
        declarations: IndexMap::new(),
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

fn get_selector(atom_name: &str, options: &Options) -> String {
    if atom_name.contains(&options.non_atom_identifier) {
        return atom_name.replace(&options.non_atom_identifier, "");
    }

    match options.atom_style {
        AtomStyle::ClassAttribute => format!(".{}", atom_name),
        AtomStyle::DataAttribute => format!("[{}=\"\"]", atom_name),
    }
}
