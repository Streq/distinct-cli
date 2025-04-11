use clap::Parser;
use distinct::get_reader;
use indexmap::IndexSet;
use std::io::BufRead;

/// Get distinct lines from stdin or file
#[derive(Parser)]
struct Args {
    /// Optional file to search
    file: Option<String>,
}
fn main() {
    let Args { file } = Args::parse();

    let reader = get_reader(file);

    let set = reader
        .lines()
        .map(|line| line.unwrap())
        .collect::<IndexSet<String>>();

    set.iter().for_each(|line| println!("{line}"));
}
