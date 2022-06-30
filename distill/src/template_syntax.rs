use indexmap::IndexMap;
use lazy_static::lazy_static;
use regex::Regex;
use serde::{Deserialize, Serialize};

use super::transformation_syntax::{
    CSSRule, CopyExistingRules, FromTokenGroup, ManyRulesFromTokenGroup, NoTransformation,
    TokenGroups, Transformation, Transformations,
};

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[serde(default)]
pub struct Options {
    pub non_atom_identifier: String,
    pub atom_style: AtomStyle,
    pub pseudo_classes: IndexMap<String, String>,
    pub breakpoints: IndexMap<String, Breakpoint>,
    pub breakpoint_modifier_style: BreakpointModifierStyle,
    pub breakpoint_modifier_seperator: String,
    pub root_variable_prefix: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum BreakpointModifierStyle {
    Prefix,
    Suffix,
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
            pseudo_classes: IndexMap::default(),
            breakpoints: IndexMap::default(),
            breakpoint_modifier_style: BreakpointModifierStyle::Prefix,
            breakpoint_modifier_seperator: "\\:".to_string(),
            root_variable_prefix: "_".to_string(),
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

#[derive(Debug)]
pub struct TemplateOptions {
    include_breakpoints: bool,
    include_pseudo_classes: bool,
}

impl Default for TemplateOptions {
    fn default() -> Self {
        Self {
            include_breakpoints: true,
            include_pseudo_classes: true,
        }
    }
}

pub fn transformations_from_tokens(tokens: &TokenGroups, options: &Options) -> Transformations {
    let mut list = Vec::new();

    for (id, _token_group) in tokens {
        let root_variable_name = format!("--{}{{{{ KEY }}}}", options.root_variable_prefix);
        let config = FromTokenGroup {
            id: format!("root-variables-{}", id),
            description: "".to_string(),
            selector: ":root".to_string(),
            token_group_name: id.to_string(),
            declarations: IndexMap::from([(root_variable_name, "{{ VAL }}".to_string())]),
        };

        list.push(Transformation::SingleRuleFromTokenGroup(config));
    }

    list
}

pub fn transformations_from_templates(
    rulesets: &Vec<CSSTemplate>,
    options: &Options,
) -> Transformations {
    let mut list = Vec::new();
    let mut ids_affected_by_breakpoints: Vec<String> = Vec::new();
    let mut ids_affected_by_pseudo_classes: Vec<String> = Vec::new();

    for ruleset in rulesets {
        let mut variable_maps: VariableMaps = IndexMap::new();
        let mut template_options = TemplateOptions::default();

        for (atom_name_template, block) in ruleset {
            match detect_template_options(atom_name_template, block, &mut template_options) {
                true => continue,
                false => (),
            }

            match detect_variable_map_declaration(atom_name_template, block, &mut variable_maps) {
                true => continue,
                false => (),
            };

            match detect_variable_map_loop(atom_name_template, block, &variable_maps, &options) {
                None => (),
                Some(config) => {
                    if template_options.include_breakpoints {
                        ids_affected_by_breakpoints.push(config.id.clone());
                    }
                    if template_options.include_pseudo_classes {
                        ids_affected_by_pseudo_classes.push(config.id.clone());
                    }
                    list.push(Transformation::NoTransformation(config));
                    continue;
                }
            }

            match detect_token_loop(atom_name_template, block, &options) {
                None => (),
                Some(config) => {
                    if template_options.include_breakpoints {
                        ids_affected_by_breakpoints.push(config.id.clone());
                    }
                    if template_options.include_pseudo_classes {
                        ids_affected_by_pseudo_classes.push(config.id.clone());
                    }
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

            if template_options.include_breakpoints {
                ids_affected_by_breakpoints.push(config.id.clone());
            }
            if template_options.include_pseudo_classes {
                ids_affected_by_pseudo_classes.push(config.id.clone());
            }

            list.push(Transformation::NoTransformation(config));
        }
    }

    let breakpoint_transforms = get_breakpoints(ids_affected_by_breakpoints, &options);
    list.extend(breakpoint_transforms);

    let pseudo_class_transforms =
        get_pseudo_class_transforms(ids_affected_by_pseudo_classes.clone(), &options);

    list.extend(pseudo_class_transforms);

    list
}

fn get_pseudo_class_transforms(
    affected_ids: Vec<String>,
    options: &Options,
) -> Vec<Transformation> {
    let mut list = vec![];

    for (name, pseudo_class) in &options.pseudo_classes {
        let mut selector = match options.atom_style {
            AtomStyle::ClassAttribute => format!("{{{{ PREV_SELECTOR_CLASS_NAME }}}}:{}", pseudo_class),
            AtomStyle::DataAttribute => format!("{{{{ PREV_SELECTOR_DATA_ATTR }}}}:{}", pseudo_class),
        };

        let sep = options.breakpoint_modifier_seperator.to_string();

        match options.breakpoint_modifier_style {
            BreakpointModifierStyle::Prefix => {
                selector = format!("{}{}{}", name, sep, selector);
            }
            BreakpointModifierStyle::Suffix => {
                selector = format!("{}{}{}", selector, sep, name);
            }
        }

        let config = CopyExistingRules {
            id: format!("pseudo-class-{}", name),
            description: "".to_string(),
            at_rule_identifier: None,
            affected_ids: affected_ids.clone(),
            new_selector: get_selector(&selector, &options),
        };

        list.push(Transformation::CopyExistingRules(config));
    }

    list
}

fn get_breakpoints(affected_ids: Vec<String>, options: &Options) -> Vec<Transformation> {
    let mut list = vec![];

    for (name, bp) in &options.breakpoints {
        let at_rule_identifier = build_media_query(&bp);

        if at_rule_identifier.is_none() {
            continue;
        }

        let mut selector = match options.atom_style {
            AtomStyle::ClassAttribute => "{{ PREV_SELECTOR_CLASS_NAME }}".to_string(),
            AtomStyle::DataAttribute => "{{ PREV_SELECTOR_DATA_ATTR }}".to_string(),
        };

        let sep = options.breakpoint_modifier_seperator.to_string();

        match options.breakpoint_modifier_style {
            BreakpointModifierStyle::Prefix => {
                selector = format!("{}{}{}", name, sep, selector);
            }
            BreakpointModifierStyle::Suffix => {
                selector = format!("{}{}{}", selector, sep, name);
            }
        }

        let config = CopyExistingRules {
            id: format!("breakpoint-{}", name),
            description: "".to_string(),
            at_rule_identifier,
            affected_ids: affected_ids.clone(),
            new_selector: get_selector(&selector, &options),
        };

        list.push(Transformation::CopyExistingRules(config));
    }

    list
}

fn build_media_query(bp: &Breakpoint) -> Option<String> {
    if bp.max_width.is_none() && bp.min_width.is_none() {
        return None;
    }

    let mut at_rule_identifier = "@media ".to_string();

    match &bp.min_width {
        None => (),
        Some(value) => {
            let value = format!("(min-width: {})", value);
            at_rule_identifier.push_str(&value);
        }
    };

    match &bp.max_width {
        None => (),
        Some(value) => {
            if bp.min_width.is_some() {
                at_rule_identifier.push_str(" and ");
            }
            let value = format!("(max-width: {})", value);
            at_rule_identifier.push_str(&value);
        }
    };

    Some(at_rule_identifier)
}

fn detect_variable_map_loop(
    atom_name_template: &str,
    block: &SugarBlock,
    variable_maps: &VariableMaps,
    options: &Options,
) -> Option<NoTransformation> {
    lazy_static! {
        static ref RE: Regex = Regex::new(r"(?P<before>.*)\[\$(?P<variable_map_name>.*)(?P<key_or_value>(\.key)|(\.value))\](?P<after>.*)").unwrap();
    }

    if false == RE.is_match(&atom_name_template) {
        return None;
    }

    let variable_map_name = RE
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

fn detect_template_options(
    atom_name_template: &str,
    block: &SugarBlock,
    template_options: &mut TemplateOptions,
) -> bool {

    if atom_name_template != "@options" {
        return false;
    }

    match block.get("include_breakpoints") {
        None => (),
        Some(value) => {
            let value = value.parse::<bool>().unwrap();
            template_options.include_breakpoints = value;
        }
    };

    match block.get("include_pseudo_classes") {
        None => (),
        Some(value) => {
            let value = value.parse::<bool>().unwrap();
            template_options.include_pseudo_classes = value;
        }
    };

    true
}

fn detect_variable_map_declaration(
    atom_name_template: &str,
    block: &SugarBlock,
    variable_maps: &mut VariableMaps,
) -> bool {
    lazy_static! {
        static ref RE: Regex = Regex::new(r"^\$(?P<variable_map_name>\S+)$").unwrap();
    }

    if false == RE.is_match(&atom_name_template) {
        return false;
    }

    let variable_map_name = RE
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
    lazy_static! {
        static ref RE: Regex = Regex::new(r"(?P<before>.*)\[\$(?P<token_group_name>.*)(?P<key_or_value>(\.key)|(\.value))\](?P<after>.*)").unwrap();
    }

    if false == RE.is_match(&atom_name_template) {
        return None;
    }

    let token_group_name = RE
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
        let root_variable_name = format!("var(--{}{{{{ KEY }}}})", options.root_variable_prefix);
        let root_variable_replacer = format!("$${}.value", token_group_name);

        let property = property_template
            .replace(&key_replacer, "{{ KEY }}")
            .replace(&value_replacer, "{{ VAL }}");

        let value = value_template
            .replace(&root_variable_replacer, &root_variable_name)
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
        return atom_name
            .replace(&options.non_atom_identifier, "")
            .trim()
            .to_string();
    }

    match options.atom_style {
        AtomStyle::ClassAttribute => format!(".{}", atom_name.trim()),
        AtomStyle::DataAttribute => format!("[{}='']", atom_name.trim()),
    }
}
