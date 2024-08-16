use clap::Parser;
use std::{
    error::Error,
    fs::File,
    io::{self, BufRead, BufReader},
};

#[derive(Parser, Debug)]
#[command(version, about, long_about=None)]
pub struct Config {
    /// Input file(s)
    #[arg(default_value = "-")]
    files: Vec<String>,

    /// Show line count
    #[arg(short = 'l', long, default_value = "false")]
    lines: bool,

    /// Show word count
    #[arg(short = 'w', long, default_value = "false")]
    words: bool,

    /// Show byte count
    #[arg(short = 'c', long, default_value = "false", group = "char")]
    bytes: bool,

    /// Show character count
    #[arg(short = 'm', long, default_value = "false", group = "char")]
    chars: bool,
}

#[derive(Debug, PartialEq)]
pub struct FileInfo {
    num_lines: usize,
    num_words: usize,
    num_bytes: usize,
    num_chars: usize,
}

type MyResult<T> = Result<T, Box<dyn Error>>;

pub fn count(mut file: impl BufRead) -> MyResult<FileInfo> {
    let mut num_lines = 0;
    let mut num_words = 0;
    let mut num_bytes = 0;
    let mut num_chars = 0;
    let mut line = String::new();

    loop {
        let line_bytes = file.read_line(&mut line)?;
        if line_bytes == 0 {
            break;
        }
        num_lines += 1;
        num_bytes += line_bytes;
        num_words += line.split_whitespace().count();
        num_chars += line.chars().count();
        line.clear();
    }

    Ok(FileInfo {
        num_lines,
        num_words,
        num_bytes,
        num_chars,
    })
}

fn print_file_info(file_info: &FileInfo, config: &Config) {
    if !(config.lines || config.words || config.bytes || config.chars) {
        print!(
            "{:>8}{:>8}{:>8}",
            file_info.num_lines, file_info.num_words, file_info.num_bytes
        );
        return;
    }
    if config.lines {
        print!("{:>8}", file_info.num_lines);
    }
    if config.words {
        print!("{:>8}", file_info.num_words);
    }
    if config.bytes {
        print!("{:>8}", file_info.num_bytes);
    }
    if config.chars {
        print!("{:>8}", file_info.num_chars);
    }
}

fn open(file_name: &str) -> MyResult<Box<dyn BufRead>> {
    match file_name {
        "-" => Ok(Box::new(BufReader::new(io::stdin()))),
        _ => Ok(Box::new(BufReader::new(File::open(file_name)?))),
    }
}

pub fn get_args() -> MyResult<Config> {
    let config = Config::parse();
    Ok(config)
}

pub fn run(config: Config) -> MyResult<()> {
    let mut total = FileInfo {
        num_lines: 0,
        num_words: 0,
        num_bytes: 0,
        num_chars: 0,
    };
    for filename in &config.files {
        match open(filename) {
            Err(err) => eprintln!("{filename}: {err}"),
            Ok(file) => {
                let result = count(file).unwrap();
                total.num_lines += result.num_lines;
                total.num_words += result.num_words;
                total.num_bytes += result.num_bytes;
                total.num_chars += result.num_chars;
                print_file_info(&result, &config);
                println!(
                    "{}",
                    if filename == "-" {
                        "".to_string()
                    } else {
                        format!(" {filename}")
                    }
                );
            }
        }
    }
    if config.files.len() > 1 {
        print_file_info(&total, &config);
        println!(" total")
    }

    Ok(())
}
