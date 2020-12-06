use std::{
    collections::{HashMap, HashSet},
    fs,
};

struct Group<'a> {
    people: Vec<Person<'a>>,
}

struct Person<'a> {
    content: &'a str,
}

impl<'a> Group<'a> {
    fn new(content: &'a str) -> Group {
        let people: Vec<_> = content
            .split("\n")
            .map(|line| Person {
                content: line.trim(),
            })
            .collect();

        Group { people }
    }

    fn distinct_yes(&self) -> usize {
        let mut set = HashSet::new();

        for person in self.people.iter() {
            for question in person.content.chars() {
                set.insert(question);
            }
        }

        return set.len();
    }

    fn everyone_yes(&self) -> usize {
        let mut map: HashMap<char, usize> = HashMap::new();

        for person in self.people.iter() {
            for question in person.content.chars() {
                let num_answers = *map.get(&question).unwrap_or(&0) + 1;
                map.insert(question, num_answers);
            }
        }

        let mut sum = 0;

        for (question, answers) in &map {
            if *answers == self.people.len() {
                sum += 1;
            }
        }

        sum
    }
}

pub fn solve() {
    let file = fs::read_to_string("inputs/day06.txt").unwrap();

    let groups: Vec<_> = file
        .split("\n\r")
        .map(|content| Group::new(content.trim()))
        .collect();

    let sum_of_counts: usize = groups.iter().map(|group| group.distinct_yes()).sum();

    println!("Sum of count: {}", sum_of_counts);

    let sum_of_counts: usize = groups.iter().map(|group| group.everyone_yes()).sum();
    println!("Sum of count where everyone agrees: {}", sum_of_counts);
}
