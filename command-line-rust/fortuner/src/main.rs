use std::ffi::OsStr;
use std::fs;
use std::path::PathBuf;

use anyhow::Result;
use anyhow::anyhow;
use anyhow::bail;
use clap::Parser;
use regex::RegexBuilder;
use walkdir::WalkDir;

#[derive(Debug, Parser)]
#[command(author, version, about)]
/// Rust version of `fortune`
struct Args {
    /// Input files or directories
    #[arg(required(true), value_name = "FILE")]
    sources: Vec<String>,

    /// Pattern
    #[arg(short('m'), long)]
    pattern: Option<String>,

    /// Case-insensitive pattern matching
    #[arg(short, long)]
    insensitive: bool,

    /// Random seed
    #[arg(short, long, value_parser(clap::value_parser!(u64)))]
    seed: Option<u64>,
}

fn main() {
    if let Err(e) = run(Args::parse()) {
        eprintln!("{e}");
        std::process::exit(1);
    }
}

fn run(args: Args) -> Result<()> {
    println!("{:#?}", args);
    let pattern = args
        .pattern
        .map(|val: String| {
            RegexBuilder::new(val.as_str())
                .case_insensitive(args.insensitive)
                .build()
                .map_err(|_| anyhow!(r#"Invalid --pattern "{val}""#))
        })
        .transpose()?;

    let files = find_files(&args.sources)?;
    println!("{:#?}", files);
    Ok(())
}

fn find_files(paths: &[String]) -> Result<Vec<PathBuf>> {
    let dat = OsStr::new("dat");
    let mut files = vec![];

    for path in paths {
        match fs::metadata(path) {
            Err(e) => bail!("{path}: {e}"),
            Ok(_) => files.extend(
                WalkDir::new(path)
                    .into_iter()
                    .filter_map(Result::ok)
                    .filter(|e| e.file_type().is_file() && e.path().extension() != Some(dat))
                    .map(|e| e.path().into()),
            ),
        }
    }

    files.sort();
    files.dedup();
    Ok(files)
}

#[cfg(test)]
mod tests {
    use super::find_files;

    #[test]
    fn test_find_files() {
        // Verify that the function finds a file known to exist
        let res = find_files(&["./tests/inputs/jokes".to_string()]);
        assert!(res.is_ok());

        let files = res.unwrap();
        assert_eq!(files.len(), 1);
        assert_eq!(
            files.get(0).unwrap().to_string_lossy(),
            "./tests/inputs/jokes"
        );

        // Fails to find a bad file
        let res = find_files(&["/path/does/not/exist".to_string()]);
        assert!(res.is_err());

        // Finds all the input files, excludes ".dat"
        let res = find_files(&["./tests/inputs".to_string()]);
        assert!(res.is_ok());

        // Check number and order of files
        let files = res.unwrap();
        assert_eq!(files.len(), 5);
        let first = files.get(0).unwrap().display().to_string();
        assert!(first.contains("ascii-art"));
        let last = files.last().unwrap().display().to_string();
        assert!(last.contains("quotes"));

        // Test for multiple sources, path must be unique and sorted
        let res = find_files(&[
            "./tests/inputs/jokes".to_string(),
            "./tests/inputs/ascii-art".to_string(),
            "./tests/inputs/jokes".to_string(),
        ]);
        assert!(res.is_ok());
        let files = res.unwrap();
        assert_eq!(files.len(), 2);
        if let Some(filename) = files.first().unwrap().file_name() {
            assert_eq!(filename.to_string_lossy(), "ascii-art".to_string())
        }
        if let Some(filename) = files.last().unwrap().file_name() {
            assert_eq!(filename.to_string_lossy(), "jokes".to_string())
        }
    }
}
