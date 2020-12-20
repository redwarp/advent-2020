use std::{
    collections::{HashMap, VecDeque},
    todo, vec,
};

use crate::files;

pub fn solve() {
    let mut numbers: Vec<_> = files::read_file_line_per_line("inputs/day10.txt")
        .into_iter()
        .map(|line| line.parse::<u32>().unwrap())
        .collect();
    numbers.push(0);
    numbers.sort();
    numbers.push(numbers.last().unwrap() + 3);

    let (diff_1, diff_3) = numbers.windows(2).fold((0, 0), |(diff_1, diff_3), window| {
        let left = window[0];
        let right = window[1];
        match right - left {
            1 => (diff_1 + 1, diff_3),
            3 => (diff_1, diff_3 + 1),
            _ => (diff_1, diff_3),
        }
    });

    let result = diff_1 * diff_3;

    println!(
        "Part 1: jolt of 1: {}, jolt of 3: {}, product: {}",
        diff_1, diff_3, result
    );

    let result = calculate_possible_plugs_2(&numbers);
    println!("Part 2: {} possibilities", result);
}

fn calculate_possible_plugs(sockets: &Vec<u32>) -> u32 {
    let mut count: u32 = 1;
    let mut pile = VecDeque::new();
    pile.push_back(0);

    'outer: while !pile.is_empty() {
        let current_index = pile.pop_front().unwrap();
        let current = sockets[current_index];
        let mut extra_path = 0;
        for next_index in current_index + 1..sockets.len() {
            let next = sockets[next_index];
            if next - current <= 3 {
                extra_path += 1;
                pile.push_back(next_index);
            } else {
                if extra_path > 0 {
                    count += extra_path - 1;
                }
                continue 'outer;
            }
        }
    }

    count
}

#[derive(Debug)]
struct Branch {
    index: usize,
    branches: Vec<usize>,
}

fn calculate_possible_plugs_2(sockets: &Vec<u32>) -> u32 {
    let mut branch_map = HashMap::new();
    // filtered_sockets.push(sockets[0]);

    let mut work_vec: Vec<usize> = vec![];
    for index in 0..sockets.len() - 1 {
        work_vec.clear();
        let current = sockets.get(index).unwrap();
        let mut count = 0;
        for index in index + 1..sockets.len() - 1 {
            let next = sockets.get(index).unwrap();
            if next - current <= 3 {
                work_vec.push(index);
                count += 1;
            }
        }
        if count > 1 {
            let branch = Branch {
                index,
                branches: work_vec.iter().copied().collect(),
            };

            branch_map.insert(index, branch);
        }
    }

    println!("Everything mapped");

    let mut other_count: u32 = 1;

    let mut keys: Vec<_> = branch_map.keys().cloned().collect();
    keys.sort();

    let mut pile = VecDeque::new();
    keys.iter().take(1).for_each(|index| pile.push_back(*index));
    while !pile.is_empty() {
        let index = pile.pop_front().unwrap();

        let first_branch = keys.iter().filter(|&key| key >= &index).nth(0);
        if let Some(first_branch) = first_branch {
            let branch = branch_map.get(first_branch).unwrap();

            other_count += branch.branches.len() as u32 - 1;
            for branch_index in branch.branches.iter() {
                pile.push_back(*branch_index);
            }
        }
    }

    // let mut count: u32 = 1;
    // let mut pile = VecDeque::new();

    // pile.push_back(0);

    // while !pile.is_empty() {
    //     let branch_index = pile.pop_front().unwrap();

    //     for index in branch_index..sockets.len() - 1 {
    //         let current = sockets.get(index).unwrap();
    //         let mut current_branch_count = 0;
    //         // Let's check if multiple branch.
    //         for next_index in index + 1..sockets.len() {
    //             let next = sockets.get(next_index).unwrap();

    //             if next - current <= 3 {
    //                 current_branch_count += 1;
    //                 if current_branch_count > 1 {
    //                     count += 1;
    //                     pile.push_back(next_index);
    //                 }
    //             } else {
    //                 break;
    //             }
    //         }
    //     }
    // }

    other_count
}
