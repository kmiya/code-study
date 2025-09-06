use std::{fs, path::PathBuf};

use anyhow::Result;
use clap::{Parser, arg};

#[derive(Debug, Parser)]
#[command(author, version, about)]
/// Rust version of `ls`
struct Args {
    /// Files and/or directories
    #[arg(default_value = ".")]
    paths: Vec<String>,

    /// Long listing
    #[arg(short, long)]
    long: bool,

    /// Show all files
    #[arg(short('a'), long("all"))]
    show_hidden: bool,
}

fn main() {
    if let Err(e) = run(Args::parse()) {
        eprintln!("{e}");
        std::process::exit(1);
    }
}

fn run(args: Args) -> Result<()> {
    let paths = find_files(&args.paths, args.show_hidden)?;
    for path in paths {
        println!("{}", path.display());
    }
    Ok(())
}

fn find_files(paths: &[String], show_hidden: bool) -> Result<Vec<PathBuf>> {
    let mut results = Vec::<PathBuf>::new();
    for path in paths {
        let metadata = fs::metadata(path);
        match metadata {
            Err(e) => eprintln!("{path}: {e}"),
            Ok(_) => {
                if metadata.unwrap().is_file() {
                    // Any existing file should be found even if hidden
                    results.push(path.into());
                    continue;
                }
                for entry in fs::read_dir(path)? {
                    let entry = entry?;
                    if entry.file_name().to_string_lossy().starts_with('.') {
                        if show_hidden {
                            results.push(entry.path());
                        }
                        continue;
                    }
                    results.push(entry.path());
                }
            }
        }
    }
    results.sort();
    Ok(results)
}

#[cfg(test)]
mod test {
    use crate::find_files;

    #[test]
    fn test_find_files() {
        // Find all non-hidden entries in a directory
        let res = find_files(&["tests/inputs".to_string()], false);
        assert!(res.is_ok());
        let mut filenames: Vec<_> = res
            .unwrap()
            .iter()
            .map(|entry| entry.display().to_string())
            .collect();
        filenames.sort();
        assert_eq!(
            filenames,
            [
                "tests/inputs/bustle.txt",
                "tests/inputs/dir",
                "tests/inputs/empty.txt",
                "tests/inputs/fox.txt",
            ]
        );

        // Any existing file should be found even if hidden
        let res = find_files(&["tests/inputs/.hidden".to_string()], false);
        assert!(res.is_ok());
        let filenames: Vec<_> = res
            .unwrap()
            .iter()
            .map(|entry| entry.display().to_string())
            .collect();
        assert_eq!(filenames, ["tests/inputs/.hidden"]);

        // Test multiple path arguments
        let res = find_files(
            &[
                "tests/inputs/bustle.txt".to_string(),
                "tests/inputs/dir".to_string(),
            ],
            false,
        );
        assert!(res.is_ok());
        let mut filenames: Vec<_> = res
            .unwrap()
            .iter()
            .map(|entry| entry.display().to_string())
            .collect();
        filenames.sort();
        assert_eq!(
            filenames,
            ["tests/inputs/bustle.txt", "tests/inputs/dir/spiders.txt"]
        );
    }

    #[test]
    fn test_find_files_hidden() {
        // Find all entries in a directory including hidden
        let res = find_files(&["tests/inputs".to_string()], true);
        assert!(res.is_ok());
        let mut filenames: Vec<_> = res
            .unwrap()
            .iter()
            .map(|entry| entry.display().to_string())
            .collect();
        filenames.sort();
        assert_eq!(
            filenames,
            [
                "tests/inputs/.hidden",
                "tests/inputs/bustle.txt",
                "tests/inputs/dir",
                "tests/inputs/empty.txt",
                "tests/inputs/fox.txt",
            ]
        );
    }
}
