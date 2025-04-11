use std::fs::File;
use std::io;
use std::io::{BufRead, BufReader};
pub fn get_reader(file: Option<String>) -> Box<dyn BufRead> {
    match file {
        Some(filename) => Box::new(BufReader::new(File::open(filename).unwrap())),
        None => Box::new(BufReader::new(io::stdin())),
    }
}
