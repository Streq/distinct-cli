use clap::Parser;
use distinct::get_reader;
use std::collections::HashSet;
use std::io::BufRead;

/// Remove lines in a that are in b
#[derive(Parser)]
struct Args {
    /// file with the filters
    filter: String,
    /// optional file to filter (otherwise it will read stdin)
    input: Option<String>,
}
fn main() {
    let Args {
        input,
        filter: exclude,
    } = Args::parse();

    let input = get_reader(input);
    let exclude = get_reader(Some(exclude));

    let set = exclude
        .lines()
        .map(|line| line.unwrap())
        .collect::<HashSet<String>>();
    input
        .lines()
        .map(|line| line.unwrap())
        .filter(|line| !set.contains(line))
        .for_each(|line| println!("{line}"));
}
