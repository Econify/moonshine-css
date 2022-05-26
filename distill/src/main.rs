
mod lib;
mod sugar;

use lib::{Transformations, TokenGroups, Intermediate};
use sugar::{SugarRuleSet, transformations_from_sugar_rules};
use serde::{Deserialize};
use std::io::BufReader;
use std::path::Path;
use std::fs;
use serde_json as json;
use serde_yaml as yaml;

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct OutputPaths {
    pub css: Option<String>,
    pub types: Option<String>,
    pub snippets: Option<String>,
    pub json: Option<String>,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct RCFile {
    pub design_tokens: Vec<String>,
    pub templates: Vec<String>,
    pub output: OutputPaths,
}

impl RCFile {
    pub fn load_from_json(path: &str) -> Self {
        let config_file = fs::File::open(&path).unwrap();
        let reader = BufReader::new(config_file);
        serde_json::from_reader(reader).unwrap()
    }
}


fn main() {
    let path_to_config = std::env::args().nth(1)
        .unwrap_or("./.moonshinerc".to_string());
    
    let config = RCFile::load_from_json(&path_to_config);

    let mut all_token_groups = TokenGroups::new();
    let mut ruleset = SugarRuleSet::new();

    for path in config.design_tokens {
        let file = fs::File::open(path).unwrap();
        let reader = BufReader::new(file);
        let token_groups: TokenGroups = yaml::from_reader(reader).unwrap();       
        for (id, token_group) in token_groups {
            all_token_groups.insert(id, token_group);
        }
    }

    for path in config.templates {
        let file = fs::File::open(path).unwrap();
        let reader = BufReader::new(file);
        let partial_ruleset: SugarRuleSet = yaml::from_reader(reader).unwrap();       
        for (atom_name_template, block) in partial_ruleset {
            ruleset.insert(atom_name_template, block);
        }
    }

    let transformations = transformations_from_sugar_rules(&ruleset);

    let intermediate = Intermediate::build(all_token_groups, transformations);
    let css = intermediate.stringify();
    let json = serde_json::to_string_pretty(&intermediate).unwrap();

    match config.output.css {
        Some(path) => write_file_creating_dirs(&path, &css),
        None => (),
    };

    match config.output.json {
        Some(path) => write_file_creating_dirs(&path, &json),
        None => (),
    };
}


fn write_file_creating_dirs(path: &str, contents: &str) {
    let path = Path::new(path);
    let parent_dir = path.clone().parent().unwrap();
    fs::create_dir_all(parent_dir).unwrap();
    fs::write(path.clone(), contents).unwrap();
}