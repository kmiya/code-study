use std::error::Error;

use clap::Parser;

#[derive(Parser, Debug)]
#[command(version, about, long_about=None)]
pub struct Config {
    /// Input file(s)
    #[arg(default_value = "-")]
    file: Vec<String>,

    /// Number lines
    #[arg(short = 'n', group = "num")]
    number: bool,

    /// Number nonblank lines
    #[arg(short = 'b', group = "num")]
    number_nonblank: bool,
}

type MyResult<T> = Result<T, Box<dyn Error>>;

pub fn get_args() -> MyResult<Config> {
    let config = Config::parse();
    Ok(config)
}

pub fn run(config: Config) -> MyResult<()> {
    dbg!(config);
    Ok(())
}
