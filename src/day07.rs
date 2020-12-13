use std::{
    cmp::Ordering,
    collections::{HashSet, VecDeque},
};

use crate::files;
use lazy_static::lazy_static;
use regex::Regex;

lazy_static! {
    static ref BAG_FILTER: Regex = Regex::new("^([a-z]+ [a-z]+) bags").unwrap();
    static ref SIMPLE_FILTER: Regex = Regex::new("([0-9]+) ([a-z]+ [a-z]+) bags?").unwrap();
}

#[derive(Debug)]
struct Bag {
    name: String,
    contained_bags: Vec<ContainedBag>,
}

impl Bag {
    fn contains(&self, other_bag: &String) -> bool {
        for bag in self.contained_bags.iter() {
            if other_bag.cmp(&bag.name) == Ordering::Equal {
                return true;
            }
        }
        return false;
    }
}

#[derive(Debug)]
struct ContainedBag {
    name: String,
    quantity: i32,
}

pub fn solve() {
    let mut bags: Vec<Bag> = vec![];

    for sample in files::read_file_line_per_line("inputs/day07.txt") {
        let name_capture = BAG_FILTER.captures(&sample).unwrap();
        let name = name_capture.get(1).unwrap().as_str().to_owned();

        let mut bag = Bag {
            name: name.clone(),
            contained_bags: vec![],
        };

        for capture in SIMPLE_FILTER.captures_iter(&sample) {
            let name = capture.get(2).unwrap().as_str().to_owned();
            let quantity: i32 = capture.get(1).unwrap().as_str().parse::<i32>().unwrap();

            bag.contained_bags.push(ContainedBag { name, quantity });
        }

        bags.push(bag);
    }

    let num_of_containing_bags = lookup_containing_bags("shiny gold", &bags);

    println!(
        "{} bags can contain shiny gold ones",
        num_of_containing_bags
    );

    let contained_in_shiny_gold = bag_count("shiny gold", &bags);
    println!(
        "Number of bags contained in shiny gold: {}",
        contained_in_shiny_gold
    );
}

fn lookup_containing_bags(name: &str, bags: &Vec<Bag>) -> usize {
    let mut lookup: VecDeque<String> = VecDeque::new();
    lookup.push_back(name.to_owned());
    let mut results: HashSet<String> = HashSet::new();

    while !lookup.is_empty() {
        let looked_up = lookup.pop_front().unwrap();

        for bag in bags.iter() {
            if bag.contains(&looked_up) {
                results.insert(bag.name.clone());
                lookup.push_back(bag.name.clone());
            }
        }
    }

    return results.len();
}

fn bag_count(name: &str, bags: &Vec<Bag>) -> i32 {
    let mut to_sum = vec![];
    let bag = bags
        .iter()
        .find(|&bag| bag.name.cmp(&name.to_owned()) == Ordering::Equal)
        .unwrap();

    for contained_bag in bag.contained_bags.iter() {
        to_sum.push((contained_bag.quantity, contained_bag.name.clone()));
    }

    to_sum
        .into_iter()
        .map(|(quantity, name)| quantity + quantity * bag_count(name.as_str(), bags))
        .sum()
}
