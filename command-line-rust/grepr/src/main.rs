use std::{
    fs::{self, File},
    io::{self, BufRead, BufReader},
    mem,
};

use anyhow::{anyhow, Result};
use clap::Parser;
use regex::{Regex, RegexBuilder};
use walkdir::WalkDir;

#[derive(Debug, Parser)]
#[command(author, version, about)]
struct Args {
    #[arg()]
    pattern: String,

    #[arg(default_value = "-", value_name = "FILE")]
    files: Vec<String>,

    #[arg(short, long)]
    insensitive: bool,

    #[arg(short, long)]
    recursive: bool,

    #[arg(short, long)]
    count: bool,

    #[arg(short('v'), long("invert-match"))]
    invert: bool,
}

fn main() {
    if let Err(e) = run(Args::parse()) {
        eprintln!("{}", e);
        std::process::exit(1);
    }
}

fn run(args: Args) -> Result<()> {
    let pattern = RegexBuilder::new(&args.pattern)
        .case_insensitive(args.insensitive)
        .build()
        .map_err(|_| anyhow!(r#"Invalid pattern "{}""#, args.pattern))?;

    let entries = find_files(&args.files, args.recursive);
    let num_files = entries.len();
    let print = |filename: &str, val: &str| {
        if num_files > 1 {
            print!("{filename}:{val}");
        } else {
            print!("{val}");
        }
    };

    for entry in entries {
        // Handle invalid entry early
        let valid_filename = match entry {
            Ok(entry) => entry,
            Err(e) => {
                eprintln!("{e}");
                continue;
            }
        };
        match open(&valid_filename) {
            Err(e) => eprintln!("filename: {e}"),
            Ok(file) => match find_lines(file, &pattern, args.invert) {
                Err(e) => eprintln!("{e}"),
                Ok(matches) => {
                    if args.count {
                        print(&valid_filename, &format!("{}\n", matches.len()));
                    } else {
                        for line in &matches {
                            print(&valid_filename, line);
                        }
                    }
                }
            },
        }
    }
    Ok(())
}

fn open(filename: &str) -> Result<Box<dyn BufRead>> {
    match filename {
        "-" => Ok(Box::new(BufReader::new(io::stdin()))),
        _ => Ok(Box::new(BufReader::new(File::open(filename)?))),
    }
}

fn find_lines<T: BufRead>(mut file: T, pattern: &Regex, invert: bool) -> Result<Vec<String>> {
    let mut matches = vec![];
    let mut line = String::new();

    loop {
        let bytes = file.read_line(&mut line)?;
        if bytes == 0 {
            break;
        }
        if pattern.is_match(&line) ^ invert {
            matches.push(mem::take(&mut line));
        }
        line.clear();
    }
    Ok(matches)
}

fn find_files(paths: &[String], recursive: bool) -> Vec<Result<String>> {
    let mut results = vec![];
    for path in paths {
        // Handle stdin case early
        if path.as_str() == "-" {
            results.push(Ok(path.to_string()));
            continue;
        }

        // Handle metadata retrieval failure early
        let metadata = match fs::metadata(path) {
            Ok(metadata) => metadata,
            Err(e) => {
                results.push(Err(anyhow!("{path}: {e}")));
                continue;
            }
        };

        if metadata.is_file() {
            results.push(Ok(path.to_string()));
        } else if metadata.is_dir() {
            if recursive {
                for entry in WalkDir::new(path)
                    .into_iter()
                    .flatten()
                    .filter(|e| e.file_type().is_file())
                {
                    results.push(Ok(entry.path().display().to_string()));
                }
            } else {
                results.push(Err(anyhow!("{path} is a directory")));
            }
        }
        // Note: Other file types (symlinks, etc.) are silently ignored
    }
    results
}

// --------------------------------------------------
#[cfg(test)]
mod tests {
    use super::{find_files, find_lines};
    use pretty_assertions::assert_eq;
    use rand::{distr::Alphanumeric, Rng};
    use regex::{Regex, RegexBuilder};
    use std::io::Cursor;

    #[test]
    fn test_find_lines() {
        let text = b"Lorem\nIpsum\r\nDOLOR";

        // The pattern _or_ should match the one line, "Lorem"
        let re1 = Regex::new("or").unwrap();
        let matches = find_lines(Cursor::new(&text), &re1, false);
        assert!(matches.is_ok());
        assert_eq!(matches.unwrap().len(), 1);

        // When inverted, the function should match the other two lines
        let matches = find_lines(Cursor::new(&text), &re1, true);
        assert!(matches.is_ok());
        assert_eq!(matches.unwrap().len(), 2);

        // This regex will be case-insensitive
        let re2 = RegexBuilder::new("or")
            .case_insensitive(true)
            .build()
            .unwrap();

        // The two lines "Lorem" and "DOLOR" should match
        let matches = find_lines(Cursor::new(&text), &re2, false);
        assert!(matches.is_ok());
        assert_eq!(matches.unwrap().len(), 2);

        // When inverted, the one remaining line should match
        let matches = find_lines(Cursor::new(&text), &re2, true);
        assert!(matches.is_ok());
        assert_eq!(matches.unwrap().len(), 1);
    }

    #[test]
    fn test_find_files() {
        // Verify that the function finds a file known to exist
        let files = find_files(&["./tests/inputs/fox.txt".to_string()], false);
        assert_eq!(files.len(), 1);
        assert_eq!(files[0].as_ref().unwrap(), "./tests/inputs/fox.txt");

        // The function should reject a directory without the recursive option
        let files = find_files(&["./tests/inputs".to_string()], false);
        assert_eq!(files.len(), 1);
        if let Err(e) = &files[0] {
            assert_eq!(e.to_string(), "./tests/inputs is a directory");
        }

        // Verify the function recurses to find four files in the directory
        let res = find_files(&["./tests/inputs".to_string()], true);
        let mut files: Vec<String> = res
            .iter()
            .map(|r| r.as_ref().unwrap().replace("\\", "/"))
            .collect();
        files.sort();
        assert_eq!(files.len(), 4);
        assert_eq!(
            files,
            vec![
                "./tests/inputs/bustle.txt",
                "./tests/inputs/empty.txt",
                "./tests/inputs/fox.txt",
                "./tests/inputs/nobody.txt",
            ]
        );

        // Generate a random string to represent a nonexistent file
        let bad: String = rand::rng()
            .sample_iter(&Alphanumeric)
            .take(7)
            .map(char::from)
            .collect();

        // Verify that the function returns the bad file as an error
        let files = find_files(&[bad], false);
        assert_eq!(files.len(), 1);
        assert!(files[0].is_err());
    }
}
