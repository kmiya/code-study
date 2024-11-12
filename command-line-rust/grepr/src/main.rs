use clap::Parser;
use regex::Regex;


#[derive(Debug, Parser)]
#[command(author, version, about)]
struct Args {
    pattern: Regex,
    files: Vec<String>,
    recursive: bool,
    count: bool,
    invert_match: bool,
}

fn main() {
    println!("Hello, world!");
}
