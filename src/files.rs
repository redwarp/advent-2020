
use std::{
    fs,
    fs::File,
    io::{prelude::*, BufReader},
    path::Path,
};

pub fn read_file_line_per_line(filename: impl AsRef<Path>) -> Vec<String> {
    let file = File::open(filename).expect("no such file");
    let buf = BufReader::new(file);
    buf.lines()
        .map(|l| l.expect("Could not parse line"))
        .collect()
}

pub fn read_file_as_csv(filename: impl AsRef<Path>) -> Vec<String> {
    let contents = fs::read_to_string(filename).expect("Could not read the file");

    contents.split(',').map(|s| s.to_string()).collect()
}
