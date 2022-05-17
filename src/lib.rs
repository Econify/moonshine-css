
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;


#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
#[serde(tag = "method")]
pub enum Transformation {
    SingleRuleFromTokenGroup(FromTokenGroup),
    ManyRulesFromTokenGroup(ManyRulesFromTokenGroup),
    CopyExistingRules(CopyExistingRules),
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct CopyExistingRules{
    id: String,
    description: String,
    affected_ids: Vec<String>,
    #[serde(rename = "@identifier")]
    at_rule_identifier: Option<String>,
    new_selector: String,
}


#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct FromTokenGroup {
    id: String,
    description: String,
    token_group_name: String,
    selector: String,
    declarations: BTreeMap<String, String>
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ManyRulesFromTokenGroup {
    id: String,
    description: String,
    token_group_name: String,
    rules: Vec<CSSRule>,
}

pub type TokenGroup = BTreeMap<String, String>;
pub type TokenGroups = BTreeMap<String, TokenGroup>;
pub type Transformations = Vec<Transformation>;


#[derive(Deserialize, Serialize, Debug, Default, Clone)]
struct CSSRule {
    selector: String,
    declarations: BTreeMap<String, String>,
}

fn err_msg_for_missing_map(token_group_name: &str) -> String {
    format!(
        "There is no input group named \"{}\"",
        token_group_name,
    )
}

fn err_msg_for_missing_transformation(description: &str, id: &str) -> String {
    format!(
        "{}: There is no transformation named {}",
        description,
        id
    )
}

/// Derive a single `CSSRule` using `FromTokenGroup`
fn many_rules_from_token_group_name(
    token_groups: &TokenGroups,
    transformation: &ManyRulesFromTokenGroup,
    intermediate: &mut Intermediate,
) {
    let token_group_name = token_groups
        .get(&transformation.token_group_name)
        .expect(&err_msg_for_missing_map(&transformation.token_group_name));

    let mut rules = vec![];

    for rule in &transformation.rules {
        for (var_key, var_val) in token_group_name {
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

    intermediate.normal_rules.insert(transformation.id.clone(), RuleFamily {
        description: transformation.description.clone(),
        css_rules: rules,
    });
}

/// Derive a single `CSSRule` using `FromTokenGroup`
fn single_rule_from_token_group_name(
    token_groups: &TokenGroups,
    transformation: &FromTokenGroup,
    intermediate: &mut Intermediate
) {
    let token_group = token_groups
        .get(&transformation.token_group_name)
        .expect(&err_msg_for_missing_map(&transformation.token_group_name));

    let selector = transformation.selector.clone();
    let mut declarations = BTreeMap::new();

    for (var_key, var_val) in token_group {
        let inject_variables = |s: &String| s
            .replace("{{ KEY }}", var_key)
            .replace("{{ VAL }}", var_val);

        for (property, value) in &transformation.declarations {
            declarations.insert(
                inject_variables(&property),
                inject_variables(&value),
            );
        }
    }

    intermediate.normal_rules.insert(transformation.id.clone(), RuleFamily {
        description: transformation.description.clone(),
        css_rules: vec![CSSRule { selector, declarations }]
    });
}

/// Copy existing rules into a media query block
fn copy_existing_rules(
    transformation: &CopyExistingRules,
    intermediate: &mut Intermediate,
) {
    let mut new_rules: Vec<CSSRule> = vec![];

    for id in &transformation.affected_ids {
        let rule_family = intermediate.normal_rules.get(&id.clone())
            .expect(&err_msg_for_missing_transformation(&transformation.description, &id));

        for rule in rule_family.css_rules.iter() {
            let mut selector = transformation.new_selector.clone();

            let prev_class_name = rule.selector.replacen(".", "", 1);
            selector = selector.replace("{{ PREV_SELECTOR_CLASS_NAME }}", &prev_class_name);
            selector = selector.replace("{{ PREV_SELECTOR }}", &rule.selector);

            new_rules.push(CSSRule {
                selector,
                ..rule.clone()
            });
        }
    }

    match &transformation.at_rule_identifier {
        Some(identifier) => {
            intermediate.at_rules.insert(transformation.id.clone(), AtRule {
                identifier: identifier.clone(),
                description: transformation.description.clone(),
                css_rules: new_rules,
            });
        },
        None => {
            intermediate.normal_rules.insert(transformation.id.clone(), RuleFamily {
                description: transformation.description.clone(),
                css_rules: new_rules,
            });
        }
    }


}

type TransformationID = String;

#[derive(Default, Serialize)]
pub struct Intermediate {
    normal_rules: BTreeMap<TransformationID, RuleFamily>,
    at_rules: BTreeMap<TransformationID, AtRule>,
}

impl Intermediate {
    pub fn build(
        token_groups: TokenGroups,
        transformations: Transformations,
    ) -> Intermediate {
        let mut intermediate = Intermediate::default();

        for transformation in &transformations {
            match transformation {
                Transformation::SingleRuleFromTokenGroup(transformation) => {
                    single_rule_from_token_group_name(&token_groups, &transformation, &mut intermediate);
                }
                Transformation::ManyRulesFromTokenGroup(transformation) => {
                    many_rules_from_token_group_name(&token_groups, &transformation, &mut intermediate);
                }
                Transformation::CopyExistingRules(transformation) => {
                    copy_existing_rules(transformation, &mut intermediate);
                }
            }
        }

        intermediate
    }

    pub fn stringify(&self) -> String {
        let mut css = String::new();

        for (_id, rule_family) in &self.normal_rules {
            let block = stringify_rules(&rule_family.css_rules);
            css = format!("{}{}", css, block);
        }

        for (_id, at_rule) in &self.at_rules {
            let mut block = stringify_rules(&at_rule.css_rules);
            block = format!("{} {{\n{}}}", at_rule.identifier, block);
            css = format!("{}{}", css, block);
        }

        css   
    }
}

#[derive(Default, Serialize)]
struct RuleFamily {
    description: String,
    css_rules: Vec<CSSRule>,
}

#[derive(Default, Serialize)]
struct AtRule {
    identifier: String,
    description: String,
    css_rules: Vec<CSSRule>,
}


fn stringify_rules(rules: &Vec<CSSRule>) -> String {
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
