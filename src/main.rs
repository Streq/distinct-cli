use clap::Parser;
use std::fs::File;
use std::io;
use std::io::{BufRead, BufReader};
use indexmap::IndexSet;
/// Get distinct lines from stdin or file
#[derive(Parser)]
struct Args {
    /// Optional file to search
    file: Option<String>,
}
fn main() {
    let args = Args::parse();

    let reader: Box<dyn BufRead> = match &args.file {
        Some(filename) => Box::new(BufReader::new(File::open(filename).unwrap())),
        None => Box::new(BufReader::new(io::stdin())),
    };

    let set = reader
        .lines()
        .map(|line| line.unwrap())
        .collect::<IndexSet<String>>();

    set.iter().for_each(|line| println!("{line}"));
}
