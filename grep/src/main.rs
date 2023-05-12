use std::env;
use std::fmt::Debug;
use std::fs::File;
use std::io::{BufRead, BufReader};
use clap::Parser;

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
}

fn main() -> Result<(), std::io::Error> {
    let args = Args::parse();

    let matcher = if args.regex {
        Matcher::Regex(regex::Regex::new(&args.match_string).unwrap())
    }
    else {
        Matcher::Literal(args.match_string)
    };

    let lines = matching_lines(&args.file_name, &matcher)?;

    for l in lines {
        println!("{}", l);
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

fn matching_lines(file_name: &str, matcher: &Matcher) -> Result<Vec<String>, std::io::Error> {
    let file = File::open(file_name)?;
    // let file = match file {
    //     Ok(f) => f,
    //     Err(e) => return Err(e),
    // };

    let mut result = Vec::new();

    for line in BufReader::new(file).lines() {
        let line = line?;
        if matcher.matches(&line) {
            result.push(line);
        }
    }

    Ok(result)
}
