use std::os::unix::fs::MetadataExt;
use std::{fs, path::PathBuf};

use anyhow::Result;
use chrono::{DateTime, Local};
use clap::{Parser, arg};
use tabular::{Row, Table};
use uzers::{get_group_by_gid, get_user_by_uid};

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
    if args.long {
        println!("{}", format_output(&paths)?);
    } else {
        for path in paths {
            println!("{}", path.display());
        }
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

fn format_output(paths: &[PathBuf]) -> Result<String> {
    //         1   2     3     4     5     6     7     8
    let fmt = "{:<}{:<}  {:>}  {:<}  {:<}  {:>}  {:<}  {:<}";
    let mut table = Table::new(fmt);

    for path in paths {
        let metadata = path.metadata()?;

        let uid = metadata.uid();
        let user = get_user_by_uid(uid)
            .map(|u| u.name().to_string_lossy().into_owned())
            .unwrap_or_else(|| uid.to_string());

        let gid = metadata.gid();
        let group = get_group_by_gid(gid)
            .map(|g| g.name().to_string_lossy().into_owned())
            .unwrap_or_else(|| gid.to_string());

        let file_type = if path.is_dir() { "d" } else { "-" };
        let perms = format_mode(metadata.mode());
        let modified: DateTime<Local> = DateTime::from(metadata.modified()?);

        table.add_row(
            Row::new()
                .with_cell(file_type) // 1
                .with_cell(perms) // 2
                .with_cell(metadata.nlink()) // 3
                .with_cell(user) // 4
                .with_cell(group) // 5
                .with_cell(metadata.len()) // 6
                .with_cell(modified.format("%b %d %y %H:%M")) // 7
                .with_cell(path.display()), // 8
        );
    }

    Ok(format!("{table}"))
}

/// Given a file mode in octal format like 0o751,
/// return a string like "rwxr-x--x"
fn format_mode(mode: u32) -> String {
    let perm =
        |digit: u32, symbol: char| -> char { if mode & digit == digit { symbol } else { '-' } };
    format!(
        "{}{}{}{}{}{}{}{}{}",
        perm(0o400, 'r'), // User
        perm(0o200, 'w'),
        perm(0o100, 'x'),
        perm(0o040, 'r'), // Group
        perm(0o020, 'w'),
        perm(0o010, 'x'),
        perm(0o004, 'r'), // Other
        perm(0o002, 'w'),
        perm(0o001, 'x'),
    )
}

#[cfg(test)]
mod test {
    use std::path::PathBuf;

    use crate::{find_files, format_mode, format_output};

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

    fn long_match(
        line: &str,
        expected_name: &str,
        expected_perms: &str,
        expected_size: Option<&str>,
    ) {
        let parts: Vec<_> = line.split_whitespace().collect();
        assert!(!parts.is_empty() && parts.len() <= 10);

        let perms = parts.first().unwrap();
        assert_eq!(perms, &expected_perms);

        if let Some(size) = expected_size {
            let file_size = parts.get(4).unwrap();
            assert_eq!(file_size, &size);
        }

        let display_name = parts.last().unwrap();
        assert_eq!(display_name, &expected_name);
    }

    #[test]
    fn test_format_output_one() {
        let bustle_path = "tests/inputs/bustle.txt";
        let bustle = PathBuf::from(bustle_path);

        let res = format_output(&[bustle]);
        assert!(res.is_ok());

        let out = res.unwrap();
        let lines: Vec<&str> = out.split('\n').filter(|s| !s.is_empty()).collect();
        assert_eq!(lines.len(), 1);

        let line1 = lines.first().unwrap();
        long_match(line1, bustle_path, "-rw-r--r--", Some("193"));
    }

    #[test]
    fn test_format_output_two() {
        let res = format_output(&[
            PathBuf::from("tests/inputs/dir"),
            PathBuf::from("tests/inputs/empty.txt"),
        ]);
        assert!(res.is_ok());

        let out = res.unwrap();
        let mut lines: Vec<&str> = out.split('\n').filter(|s| !s.is_empty()).collect();
        lines.sort();
        assert_eq!(lines.len(), 2);

        let empty_line = lines.remove(0);
        long_match(
            empty_line,
            "tests/inputs/empty.txt",
            "-rw-r--r--",
            Some("0"),
        );

        let dir_line = lines.remove(0);
        long_match(dir_line, "tests/inputs/dir", "drwxr-xr-x", None);
    }

    #[test]
    fn test_format_mode() {
        assert_eq!(format_mode(0o755), "rwxr-xr-x");
        assert_eq!(format_mode(0o421), "r---w---x");
    }
}
