use clap::Parser;
use distinct::get_reader;
use std::io::BufRead;

/// Join lines
#[derive(Parser)]
struct Args {
    /// Optional file to join (will use stdin otherwise)
    file: Option<String>,
    #[arg(short = 's', long, default_value = ",")]
    separator: String,
}
fn main() {
    let Args { file, separator } = Args::parse();

    let reader = get_reader(file);

    let mut it = reader.lines().map(|line| line.unwrap()).peekable();

    while let Some(line) = it.next() {
        print!("{line}");
        if it.peek().is_none() {
            println!();
        } else {
            print!("{separator}");
        }
    }
}
