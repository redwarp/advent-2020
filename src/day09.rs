use crate::files;

pub fn solve() {
    let lines = files::read_file_line_per_line("inputs/day09.txt");

    let numbers: Vec<_> = lines
        .into_iter()
        .map(|line| line.parse::<u64>().unwrap())
        .collect();

    let invalid = find_invalid_number(&numbers, 25);

    println!("First invalid: {:?}", invalid);

    if let Some(invalid) = invalid {
        let weakness = find_encryption_weakness(invalid, &numbers);

        println!("Weakeness: {:?}", weakness);
    }
}

fn find_invalid_number(numbers: &Vec<u64>, preamble: usize) -> Option<u64> {
    'outer: for index in preamble..numbers.len() {
        let number = numbers[index];

        for index_left in (index - preamble)..(index - 1) {
            let number_left = numbers[index_left];

            for index_right in (index_left + 1)..index {
                let number_right = numbers[index_right];

                if number_left + number_right == number {
                    continue 'outer;
                }
            }
        }
        return Some(number);
    }

    None
}

fn find_encryption_weakness(number: u64, numbers: &Vec<u64>) -> Option<u64> {
    'outer: for index in 0..numbers.len() {
        let mut accumulator = 0;

        for summed_index in index..numbers.len() {
            accumulator += numbers[summed_index];

            if accumulator > number {
                continue 'outer;
            } else if accumulator == number {
                // We have a match! Lets lookup the smallest and biggest.
                let smallest = &numbers[index..summed_index + 1]
                    .iter()
                    .min()
                    .unwrap_or(&0)
                    .clone();
                let biggest = &numbers[index..summed_index + 1]
                    .iter()
                    .max()
                    .unwrap_or(&0)
                    .clone();
                return Some(smallest + biggest);
            }
        }
    }
    None
}
