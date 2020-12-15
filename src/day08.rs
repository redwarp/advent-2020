use std::convert::TryInto;

use crate::files;

#[derive(Debug, Clone, Copy)]
struct Line {
    instruction: Instruction,
    steps: i32,
    visited: bool,
}

#[derive(Debug, Clone, Copy, PartialEq)]
enum Instruction {
    Acc,
    Jmp,
    Nop,
}

pub fn solve() {
    let lines = files::read_file_line_per_line("inputs/day08.txt");

    let mut lines: Vec<_> = lines
        .into_iter()
        .map(|line| {
            let parts: Vec<&str> = line.split(" ").collect();
            let mut steps = parts[1][1..].parse::<i32>().unwrap();
            if parts[1].starts_with("-") {
                steps = -1 * steps;
            }
            let instruction = match parts[0] {
                "acc" => Instruction::Acc,
                "jmp" => Instruction::Jmp,
                _ => Instruction::Nop,
            };

            Line {
                instruction,
                steps,
                visited: false,
            }
        })
        .collect();

    let mut instruction_index: i32 = 0;
    let mut accumulator = 0;

    loop {
        let index: Result<usize, _> = instruction_index.try_into();
        if index.is_err() {
            break;
        }
        let index = index.unwrap();

        let line = lines.get_mut(index);
        if let Some(line) = line {
            if line.visited {
                break;
            } else {
                line.visited = true;
                match line.instruction {
                    Instruction::Acc => {
                        accumulator += line.steps;
                        instruction_index += 1;
                    }
                    Instruction::Jmp => {
                        instruction_index += line.steps;
                    }
                    Instruction::Nop => {
                        instruction_index += 1;
                    }
                }
            }
        } else {
            break;
        }
    }

    println!("Value in accumulator: {}", accumulator);

    lines.iter_mut().for_each(|line| line.visited = false);

    let mut modified_index: Option<usize> = None;
    let mut interrupted = true;

    while interrupted {
        interrupted = false;
        let mut lines = lines.clone();
        let initial_index = modified_index.unwrap_or(0);

        for index in initial_index..lines.len() {
            if modified_index.is_none() || index > initial_index {
                let mut line = lines.get_mut(index).unwrap();

                if line.instruction == Instruction::Nop {
                    line.instruction = Instruction::Jmp;
                    modified_index = Some(index);
                    break;
                } else if line.instruction == Instruction::Jmp {
                    line.instruction = Instruction::Nop;
                    modified_index = Some(index);
                    break;
                }
            }
        }

        instruction_index = 0;
        accumulator = 0;
        loop {
            let index: Result<usize, _> = instruction_index.try_into();
            if index.is_err() {
                interrupted = true;
                break;
            }
            let index = index.unwrap();

            let line = lines.get_mut(index);
            if let Some(line) = line {
                if line.visited {
                    interrupted = true;
                    break;
                } else {
                    line.visited = true;
                    match line.instruction {
                        Instruction::Acc => {
                            accumulator += line.steps;
                            instruction_index += 1;
                        }
                        Instruction::Jmp => {
                            instruction_index += line.steps;
                        }
                        Instruction::Nop => {
                            instruction_index += 1;
                        }
                    }
                }
            } else {
                if index >= lines.len() {
                    // End of program
                    break;
                } else {
                    interrupted = true;
                    break;
                }
            }
        }
    }

    println!(
        "After modifying instruction {}, we exited the loop. Acc is {}",
        modified_index.unwrap_or(0),
        accumulator
    );
}
