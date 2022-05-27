mod init;
mod io;
mod template_syntax;
mod transformation_syntax;

use clap::Parser;
use init::initialize_moonshinerc;
use io::write_file_creating_dirs;
use serde::Deserialize;
use serde_yaml as yaml;
use std::fs;
use std::io::BufReader;
use std::path::{Path, PathBuf};
use template_syntax::{transformations_from_templates, CSSTemplate, Options};
use transformation_syntax::{Intermediate, TokenGroups};

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    /// Sets a custom config file
    #[clap(short, long, parse(from_os_str), value_name = "FILE")]
    config: Option<PathBuf>,

    /// Initialize .moonshinerc
    #[clap(short, long, parse(from_occurrences))]
    init: usize,

    /// Enable watcher mode
    #[clap(short, long, parse(from_occurrences))]
    watch: usize,

    /// Turn debugging information on
    #[clap(short, long, parse(from_occurrences))]
    debug: usize,
}

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
    pub options: Options,
    pub design_tokens: Vec<String>,
    pub templates: Vec<String>,
    pub output: OutputPaths,
}

impl RCFile {
    pub fn load_from_json(path: &str) -> Self {
        let rc_file_file = fs::File::open(&path).unwrap();
        let reader = BufReader::new(rc_file_file);
        serde_json::from_reader(reader).unwrap()
    }
}

fn main() {
    let args = Args::parse();
    let _debug_enabled = args.debug > 0;
    let _watch_enabled = args.watch > 0;

    let path_to_rc_file = args
        .config
        .as_deref()
        .unwrap_or(Path::new("./.moonshinerc"));

    if args.init != 0 {
        initialize_moonshinerc(&path_to_rc_file.to_str().unwrap());
        std::process::exit(0);
    }

    let rc_file = RCFile::load_from_json(&path_to_rc_file.to_str().unwrap());

    let mut all_token_groups = TokenGroups::new();
    let mut ruleset = CSSTemplate::new();

    for path in rc_file.design_tokens {
        let file = fs::File::open(path).unwrap();
        let reader = BufReader::new(file);
        let token_groups: TokenGroups = yaml::from_reader(reader).unwrap();
        for (id, token_group) in token_groups {
            all_token_groups.insert(id, token_group);
        }
    }

    for path in rc_file.templates {
        let file = fs::File::open(path).unwrap();
        let reader = BufReader::new(file);
        let partial_ruleset: CSSTemplate = yaml::from_reader(reader).unwrap();
        for (atom_name_template, block) in partial_ruleset {
            ruleset.insert(atom_name_template, block);
        }
    }

    let transformations = transformations_from_templates(&ruleset, &rc_file.options);

    let intermediate = Intermediate::build(all_token_groups, transformations);
    let css = intermediate.stringify();
    let json = serde_json::to_string_pretty(&intermediate).unwrap();

    match rc_file.output.css {
        None => (),
        Some(path) => match write_file_creating_dirs(&path, &css) {
            Err(why) => panic!("{}", why),
            Ok(_) => (),
        },
    };

    match rc_file.output.json {
        None => (),
        Some(path) => match write_file_creating_dirs(&path, &json) {
            Err(why) => panic!("{}", why),
            Ok(_) => (),
        },
    };
}
