use clap::builder::TypedValueParser;
use clap::{Arg, ArgAction, Command};
use std::error::Error;
use std::fs::File;
use std::io::{self, BufRead, BufReader};

#[derive(Debug)]
pub struct Config {
    files: Vec<String>,
    lines: usize,
    bytes: Option<usize>,
}

type MyResult<T> = Result<T, Box<dyn Error>>;

pub fn get_args() -> MyResult<Config> {
    let matches = Command::new("headr")
        .version("0.1.0")
        .author("Sam Hepburn")
        .about("Rust head")
        .arg(
            Arg::new("file")
                .value_name("FILE")
                .help("file")
                .num_args(1..)
                .default_value("-"),
        )
        .arg(
            Arg::new("count")
                .short('n')
                .long("count")
                .help("the number of lines to return")
                .conflicts_with("bytes")
                .default_value("10"),
        )
        .arg(
            Arg::new("bytes")
                .short('c')
                .long("bytes")
                .help("number output non blank"),
        )
        .get_matches();

    Ok(Config {
        files: matches
            .get_many("file")
            .expect("a file is required")
            .cloned()
            .collect(),
        lines: matches.get_one::<usize>("count").unwrap().clone(),
        bytes: matches.get_one::<usize>("bytes").copied(),
    })
}

pub fn run(config: Config) -> MyResult<()> {
    println!("{:#?}", config);
    Ok(())
}

pub fn parse_positive_int(val: &str) -> MyResult<usize> {
    match val.parse() {
        Ok(v) if v > 0 => Ok(v),
        _ => Err(From::from(val))
    }
}

#[test]
fn test_parse_positive_int() {
    // 3 is an OK integer
    let res = parse_positive_int("3");
    assert!(res.is_ok());
    assert_eq!(res.unwrap(), 3);

    // Any string is an error
    let res = parse_positive_int("foo");
    assert!(res.is_err());
    assert_eq!(res.unwrap_err().to_string(), "foo".to_string());

    // Zero is an error
    let res = parse_positive_int("0");
    assert!(res.is_err());
    assert_eq!(res.unwrap_err().to_string(), "0".to_string());
}