use crate::TakeValue::*;
use std::sync::OnceLock;

use anyhow::{Result, anyhow, bail};
use clap::{Parser, arg};
use regex::Regex;

#[derive(Debug, Parser)]
#[command(author, version, about)]
/// Rust version of `tail`
struct Args {
    /// Input file(s)
    #[arg(required = true)]
    files: Vec<String>,

    /// Number of lines
    #[arg(value_name = "LINES", short('n'), long, default_value = "10")]
    lines: String,

    /// Number of bytes
    #[arg(value_name = "BYTES", short('c'), long, conflicts_with("lines"))]
    bytes: Option<String>,

    /// Suppress headers
    #[arg(short, long)]
    quiet: bool,
}

#[derive(Debug, PartialEq)]
enum TakeValue {
    PlusZero,
    TakeNum(i64),
}

fn main() {
    if let Err(e) = run(Args::parse()) {
        eprintln!("{e}");
        std::process::exit(1);
    }
}

fn run(args: Args) -> Result<()> {
    let lines = parse_num(args.lines).map_err(|e| anyhow!("illegal line count -- {e}"))?;
    let bytes = args
        .bytes
        .map(parse_num)
        .transpose()
        .map_err(|e| anyhow!("illegal byte count -- {e}"))?;
    Ok(())
}

static NUM_RE: OnceLock<Regex> = OnceLock::new();

fn parse_num(val: String) -> Result<TakeValue> {
    let num_re = NUM_RE.get_or_init(|| Regex::new(r"^([-+])?(\d+)$").unwrap());
    match num_re.captures(&val) {
        Some(caps) => {
            let sign = caps.get(1).map_or("-", |m| m.as_str());
            let signed_num = format!("{sign}{}", caps.get(2).unwrap().as_str());
            if let Ok(num) = signed_num.parse() {
                if sign == "+" && num == 0 {
                    Ok(TakeValue::PlusZero)
                } else {
                    Ok(TakeNum(num))
                }
            } else {
                bail!(val)
            }
        }
        _ => bail!(val),
    }
}

#[cfg(test)]
mod tests {
    use crate::TakeValue::PlusZero;
    use crate::TakeValue::TakeNum;
    use crate::parse_num;

    #[test]
    fn test_parse_num() {
        // All integers should be interpreted as negative numbers
        let res = parse_num("3".to_string());
        assert!(res.is_ok());
        assert_eq!(res.unwrap(), TakeNum(-3));

        // A leading "+" should result in a positive number
        let res = parse_num("+3".to_string());
        assert!(res.is_ok());
        assert_eq!(res.unwrap(), TakeNum(3));

        // An explicit "-" value should result in a negative number
        let res = parse_num("-3".to_string());
        assert!(res.is_ok());
        assert_eq!(res.unwrap(), TakeNum(-3));

        // Zero is zero
        let res = parse_num("0".to_string());
        assert!(res.is_ok());
        assert_eq!(res.unwrap(), TakeNum(0));

        // Plus zero is special
        let res = parse_num("+0".to_string());
        assert!(res.is_ok());
        assert_eq!(res.unwrap(), PlusZero);

        // Test boundaries
        let res = parse_num(i64::MAX.to_string());
        assert!(res.is_ok());
        assert_eq!(res.unwrap(), TakeNum(i64::MIN + 1));

        let res = parse_num((i64::MIN + 1).to_string());
        assert!(res.is_ok());
        assert_eq!(res.unwrap(), TakeNum(i64::MIN + 1));

        let res = parse_num(format!("+{}", i64::MAX));
        assert!(res.is_ok());
        assert_eq!(res.unwrap(), TakeNum(i64::MAX));

        let res = parse_num(i64::MIN.to_string());
        assert!(res.is_ok());
        assert_eq!(res.unwrap(), TakeNum(i64::MIN));

        // A floating-point value is invalid
        let res = parse_num("3.14".to_string());
        assert!(res.is_err());
        assert_eq!(res.unwrap_err().to_string(), "3.14");

        // Any non-integer string is invalid
        let res = parse_num("foo".to_string());
        assert!(res.is_err());
        assert_eq!(res.unwrap_err().to_string(), "foo");
    }
}
