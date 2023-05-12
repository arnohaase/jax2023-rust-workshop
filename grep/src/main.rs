mod word_statistics;

use std::env;
use std::fmt::Debug;
use std::fs::File;
use std::io::{BufRead, BufReader};
use clap::Parser;
use log::LevelFilter;
use simple_logger::SimpleLogger;
use crate::word_statistics::*;

#[derive(Parser)]
#[clap(name="my-grep")]
struct Args {
    #[clap(short, long)]
    /// The file name to be searched
    file_name: String,

    #[clap(short, long)]
    match_string: String,

    #[clap(short, long)]
    regex: bool,

    #[clap(short, long, default_value="3")]
    top_n: usize,
}

fn main() -> Result<(), std::io::Error> {
    SimpleLogger::new()
        .with_colors(true)
        .with_level(LevelFilter::Debug)
        .init()
        .unwrap();

    let args = Args::parse();

    let matcher = if args.regex {
        Matcher::Regex(regex::Regex::new(&args.match_string).unwrap())
    }
    else {
        Matcher::Literal(args.match_string)
    };

    let mut statistics = WordStatistics::default();

    let lines = matching_lines(&args.file_name, &matcher, &mut statistics)?;

    for l in lines {
        println!("{}", l);
    }

    println!("--- top {} ---", args.top_n);

    for (word, num) in statistics.top_n(args.top_n) {
        println!("{}: {}", word.0, num)
    }

    Ok(())
}

enum Matcher {
    Literal(String),
    Regex(regex::Regex),
}
impl Matcher {
    fn matches(&self, line: &str) -> bool {
        match self {
            Matcher::Literal(s) => line.contains(s),
            Matcher::Regex(r) => r.is_match(line),
        }
    }
}

fn matching_lines(file_name: &str, matcher: &Matcher, statistics: &mut WordStatistics) -> Result<Vec<String>, std::io::Error> {
    let file = File::open(file_name)?;
    let mut result = Vec::new();

    for line in BufReader::new(file).lines() {
        let line = line?;
        statistics.add_line(&line);
        if matcher.matches(&line) {
            result.push(line);
        }
    }

    Ok(result)
}
