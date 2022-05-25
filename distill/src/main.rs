mod init;
mod io;
mod lib;

use clap::Parser;
use init::initialize_moonshinerc;
use io::write_file_creating_dirs;
use lib::{Intermediate, TokenGroups, Transformations};
use serde::Deserialize;
use std::fs;
use std::io::BufReader;
use std::path::{Path, PathBuf};

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
    pub design_tokens: Vec<String>,
    pub transformations: Vec<String>,
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
    let args = Args::parse();
    let _debug_enabled = args.debug > 0;
    let _watch_enabled = args.watch > 0;

    let value = args
        .config
        .as_deref()
        .unwrap_or(Path::new("./.moonshinerc"));

    if args.init != 0 {
        initialize_moonshinerc(&value.to_str().unwrap());
        std::process::exit(0);
    }

    let config = RCFile::load_from_json(&value.to_str().unwrap());

    let mut all_token_groups = TokenGroups::new();

    for path in config.design_tokens {
        let file = fs::File::open(path).unwrap();
        let reader = BufReader::new(file);
        let token_groups: TokenGroups = serde_json::from_reader(reader).unwrap();       
        for (id, token_group) in token_groups {
            all_token_groups.insert(id, token_group);
        }
    }

    let mut all_transformations = Transformations::new();

    for path in config.transformations {
        let file = fs::File::open(path).unwrap();
        let reader = BufReader::new(file);
        let transformations: Transformations = serde_json::from_reader(reader).unwrap();       
        for transformation in transformations {
            all_transformations.push(transformation);
        }
    }

    let intermediate = Intermediate::build(all_token_groups, all_transformations);
    let css = intermediate.stringify();
    let json = serde_json::to_string_pretty(&intermediate).unwrap();

    match config.output.css {
        None => (),
        Some(path) => match write_file_creating_dirs(&path, &css) {
            Err(why) => panic!("{}", why),
            Ok(_) => (),
        },
    };

    match config.output.json {
        None => (),
        Some(path) => match write_file_creating_dirs(&path, &json) {
            Err(why) => panic!("{}", why),
            Ok(_) => (),
        },
    };
}
