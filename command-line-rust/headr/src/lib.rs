use std::{
    error::Error,
    fs::File,
    io::{self, BufRead, BufReader},
};

use clap::Parser;

#[derive(Parser, Debug)]
#[command(version, about, long_about=None)]
pub struct Config {
    /// Input file(s)
    #[arg(default_value = "-")]
    files: Vec<String>,

    /// Number lines
    #[arg(short = 'n', long, default_value = "10", group = "num")]
    lines: usize,

    /// Number nonblank lines
    #[arg(short = 'c', long, group = "num")]
    bytes: Option<usize>,
}

type MyResult<T> = Result<T, Box<dyn Error>>;

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
    for file_name in config.files {
        match open(&file_name) {
            Err(err) => eprintln!("{file_name}: {err}"),
            Ok(mut file) => {
                match config.bytes {
                    Some(read_bytes) => {
                        let mut buf = vec![0; read_bytes];
                        file.read_exact(&mut buf)?;
                        print!("{}", String::from_utf8_lossy(&buf));
                        break;
                    }
                    None => (),
                }
                let print_lines = config.lines;
                for (line_num, line_result) in file.lines().enumerate() {
                    if print_lines < line_num + 1 {
                        break;
                    }
                    let line = line_result?;
                    println!("{line}");
                }
            }
        }
    }
    Ok(())
}
