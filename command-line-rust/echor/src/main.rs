use clap::Parser;

#[derive(Parser)]
#[command(version, about, long_about=None)]
struct Cli {
    /// Input text
    text: Vec<String>,

    /// Do not print newline
    #[arg(short)]
    n: bool,
}

fn main() {
    let cli = Cli::parse();
    print!("{}{}", cli.text.join(" "), if cli.n { "" } else { "\n" });
}
