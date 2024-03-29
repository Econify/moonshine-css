mod errors;
mod init;
mod io;
mod template_syntax;
mod transformation_syntax;

use clap::Parser;
use errors::Exit;
use init::initialize_moonshinerc;
use io::write_file_creating_dirs;
use serde::Deserialize;
use serde_json as json;
use serde_yaml as yaml;
use std::fs;
use std::io::BufReader;
use std::path::{Path, PathBuf};
use std::time::Instant;
use template_syntax::{
    transformations_from_templates, transformations_from_tokens, CSSTemplate, Options,
};
use transformation_syntax::{Intermediate, TokenGroups};

const DEFAULT_RC_FILE_NAME: &str = ".moonshinerc";

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
    pub css_variables: Option<String>,
    pub css_atoms: Option<String>,
    pub json_atoms: Option<String>,
    pub types: Option<String>,
    pub snippets: Option<String>,
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
    pub fn load_from_json(path: &Path) -> Self {
        let rc_file_handle = fs::File::open(&path).unwrap_or_else(|err| {
            let message = &errors::describe_rc_file_open_error(err, DEFAULT_RC_FILE_NAME);
            Exit::with_message(message)
        });

        json::from_reader(BufReader::new(rc_file_handle)).unwrap_or_else(|err| {
            Exit::with_message(&format!("Failed parse RC File as JSON: {}.", err))
        })
    }
}

fn main() {
    let args = Args::parse();
    let _debug_enabled = args.debug > 0;
    let _watch_enabled = args.watch > 0;
    let exec_start = Instant::now();

    let path_to_rc_file = args
        .config
        .as_deref()
        .unwrap_or(Path::new(DEFAULT_RC_FILE_NAME));

    if args.init != 0 {
        initialize_moonshinerc(&path_to_rc_file);
        std::process::exit(0);
    }

    let rc_file = RCFile::load_from_json(&path_to_rc_file);

    let mut all_token_groups = TokenGroups::new();
    let mut rulesets = Vec::new();

    for path in rc_file.design_tokens {
        let file = fs::File::open(path.clone()).unwrap_or_else(|_err| {
            Exit::with_message(&format!("Failed to open design token file: {}", path))
        });

        let reader = BufReader::new(file);

        let token_groups: TokenGroups = yaml::from_reader(reader).unwrap_or_else(|err| {
            Exit::with_message(&format!("Failed to parse `{}` as yaml: {}", path, err))
        });

        for (id, token_group) in token_groups {
            all_token_groups.insert(id, token_group);
        }
    }

    for path in rc_file.templates {
        let file = fs::File::open(path.clone()).unwrap_or_else(|_err| {
            Exit::with_message(&format!("Failed to open template file: {}", path))
        });

        let reader = BufReader::new(file);

        let ruleset: CSSTemplate = yaml::from_reader(reader).unwrap_or_else(|err| {
            Exit::with_message(&format!("Failed to parse `{}` as yaml: {}", path, err))
        });

        rulesets.push(ruleset);
    }

    // Global Variables
    let root_transformations = transformations_from_tokens(&all_token_groups, &rc_file.options);
    let root_intermediate = Intermediate::build(all_token_groups.clone(), root_transformations);
    let root_css = root_intermediate.stringify();

    // Atomic Styles
    let transformations = transformations_from_templates(&rulesets, &rc_file.options);
    let intermediate = Intermediate::build(all_token_groups, transformations);
    let css = intermediate.stringify();

    let json = serde_json::to_string_pretty(&intermediate)
        .unwrap_or_else(|err| Exit::with_message(&format!("Failed to stringify JSON: {}", err)));

    match rc_file.output.css_variables {
        None => (),
        Some(path) => match write_file_creating_dirs(&path, &root_css) {
            Err(why) => panic!("{}", why),
            Ok(_) => (),
        },
    };

    match rc_file.output.css_atoms {
        None => (),
        Some(path) => match write_file_creating_dirs(&path, &css) {
            Err(why) => panic!("{}", why),
            Ok(_) => (),
        },
    };

    match rc_file.output.json_atoms {
        None => (),
        Some(path) => match write_file_creating_dirs(&path, &json) {
            Err(why) => panic!("{}", why),
            Ok(_) => (),
        },
    };

    let exec_duration = exec_start.elapsed();
    println!("✅ Done [{:?}]", exec_duration);
}
