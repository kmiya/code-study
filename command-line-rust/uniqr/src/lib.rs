use clap::Parser;
use std::{
    error::Error,
    fs::File,
    io::{self, BufRead, BufReader, Write},
};

type MyResult<T> = Result<T, Box<dyn Error>>;

#[derive(Parser, Debug)]
#[command(version, about, long_about=None)]
pub struct Config {
    /// Input file
    #[arg(default_value = "-")]
    in_file: String,

    /// Output file
    out_file: Option<String>,

    /// Show counts
    #[arg(short = 'c', long)]
    count: bool,
}

pub fn get_args() -> MyResult<Config> {
    let config = Config::parse();
    Ok(config)
}

fn open(file_name: &str) -> MyResult<Box<dyn BufRead>> {
    match file_name {
        "-" => Ok(Box::new(BufReader::new(io::stdin()))),
        _ => Ok(Box::new(BufReader::new(File::open(file_name)?))),
    }
}

pub fn run(config: Config) -> MyResult<()> {
    let mut file = open(&config.in_file).map_err(|e| format!("{}: {}", config.in_file, e))?;
    let mut line = String::new();
    let mut prev_line = String::new();
    let mut count: u64 = 0;
    let mut first_line = true;
    let mut out_file: Box<dyn Write> = match &config.out_file {
        Some(out_name) => Box::new(File::create(out_name)?),
        _ => Box::new(io::stdout()),
    };
    loop {
        let bytes = file.read_line(&mut line)?;
        if bytes == 0 {
            break;
        }
        if first_line {
            first_line = false;
            prev_line = line.clone();
            count += 1;
            line.clear();
            continue;
        }
        if prev_line.trim_end() == line.trim_end() {
            count += 1;
            line.clear();
            continue;
        }
        if config.count {
            write!(out_file, "{:>4} {prev_line}", count)?;
        } else {
            write!(out_file, "{prev_line}")?;
        }
        prev_line = line.clone();
        line.clear();
        count = 1;
    }
    if count > 0 {
        if config.count {
            write!(out_file, "{:>4} {prev_line}", count)?;
        } else {
            write!(out_file, "{prev_line}")?;
        }
    }
    Ok(())
}
