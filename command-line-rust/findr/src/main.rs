use anyhow::Result;
use clap::{builder::PossibleValue, ArgAction, Parser, ValueEnum};
use regex::Regex;
use walkdir::WalkDir;

#[derive(Parser, Debug)]
#[command(version, about, long_about=None)]
pub struct Args {
    /// Search path(s)
    #[arg(default_value = ".", value_name = "PATH")]
    paths: Vec<String>,

    /// Names
    #[arg(
        short = 'n',
        long = "name",
        value_parser = Regex::new,
        value_name = "NAME",
        action=ArgAction::Append,
        num_args = 0..
    )]
    names: Vec<Regex>,

    /// Entry type
    #[arg(
        short = 't',
        long = "type",
        value_name = "TYPE",
        value_parser = clap::value_parser!(EntryType),
        action = ArgAction::Append,
        num_args = 0..
    )]
    entry_types: Vec<EntryType>,
}

#[derive(Debug, PartialEq, Eq, Clone)]
enum EntryType {
    Dir,
    File,
    Link,
}

impl ValueEnum for EntryType {
    fn value_variants<'a>() -> &'a [Self] {
        &[EntryType::Dir, EntryType::File, EntryType::Link]
    }

    fn to_possible_value<'a>(&self) -> Option<PossibleValue> {
        Some(match self {
            EntryType::Dir => PossibleValue::new("d"),
            EntryType::File => PossibleValue::new("f"),
            EntryType::Link => PossibleValue::new("l"),
        })
    }
}

fn main() {
    if let Err(e) = run(Args::parse()) {
        eprintln!("{}", e);
        std::process::exit(1);
    }
}

pub fn run(args: Args) -> Result<()> {
    for path in args.paths {
        for entry in WalkDir::new(path) {
            match entry {
                Err(e) => eprintln!("{e}"),
                Ok(entry) => {
                    if !args.names.is_empty() {
                        if !args
                            .names
                            .iter()
                            .any(|x| x.is_match(&entry.file_name().to_string_lossy()))
                        {
                            continue;
                        }
                    }
                    if args.entry_types.is_empty() {
                        println!("{}", entry.path().display());
                        continue;
                    }
                    if args.entry_types.contains(&EntryType::Dir) {
                        if entry.file_type().is_dir() {
                            println!("{}", entry.path().display());
                        }
                    }
                    if args.entry_types.contains(&EntryType::File) {
                        if entry.file_type().is_file() {
                            println!("{}", entry.path().display());
                        }
                    }
                    if args.entry_types.contains(&EntryType::Link) {
                        if entry.file_type().is_symlink() {
                            println!("{}", entry.path().display());
                        }
                    }
                }
            }
        }
    }
    Ok(())
}
