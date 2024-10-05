use clap::{Parser, ValueEnum};
use regex::Regex;
use std::error::Error;

type MyResult<T> = Result<T, Box<dyn Error>>;

#[derive(Debug, PartialEq, Eq, Clone, ValueEnum)]
enum EntryType {
    D,
    F,
    L,
}

#[derive(Parser, Debug)]
#[command(version, about, long_about=None)]
pub struct Config {
    /// Name
    #[arg(short = 'n', long = "name", value_name = "NAME")]
    names: Vec<Regex>,

    /// Entry type
    #[arg(short = 't', long = "type")]
    entry_types: Vec<EntryType>,

    /// Search paths
    #[arg(default_value = ".", value_name = "PATH")]
    paths: Vec<String>,
}

pub fn get_args() -> MyResult<Config> {
    let config = Config::parse();
    Ok(config)
}

pub fn run(config: Config) -> MyResult<()> {
    println!("{:?}", config);
    Ok(())
}
