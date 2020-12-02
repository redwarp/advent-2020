use regex::Regex;

use crate::files::read_file_line_per_line;

struct Password {
    min: usize,
    max: usize,
    letter: char,
    password: String,
}

impl Password {
    fn from(input: String, regex: &Regex) -> Self {
        let captured = regex.captures(&input).unwrap();
        Password {
            min: captured[1].parse::<usize>().unwrap(),
            max: captured[2].parse::<usize>().unwrap(),
            letter: captured[3].chars().next().unwrap(),
            password: captured[4].to_owned(),
        }
    }

    fn is_valid(&self) -> bool {
        let char_count = self
            .password
            .chars()
            .filter(|letter| letter == &self.letter)
            .count();

        char_count >= self.min && char_count <= self.max
    }

    fn is_valid_2(&self) -> bool {
        let result = (self.password.as_bytes()[self.min - 1] as char == self.letter)
            ^ (self.password.as_bytes()[self.max - 1] as char == self.letter);
        result
    }
}

pub fn solve() {
    let inputs = read_file_line_per_line("inputs/day02.txt");

    let regex = Regex::new("([0-9]+)-([0-9]+) ([a-z]): ([a-z]+)").unwrap();

    let inputs: Vec<_> = inputs
        .into_iter()
        .map(|input| Password::from(input, &regex))
        .collect();

    let valid_count = inputs.iter().filter(|password| password.is_valid()).count();
    println!("Valid password 1: {}", valid_count);

    let valid_count = inputs
        .iter()
        .filter(|password| password.is_valid_2())
        .count();
    println!("Valid password 2: {}", valid_count);
}
