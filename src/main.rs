mod day02;
mod day03;
mod day04;
mod day05;
mod files;

fn main() {
    let days: Vec<fn()> = vec![
        day01,
        crate::day02::solve,
        crate::day03::solve,
        crate::day04::solve,
        crate::day05::solve,
    ];

    for (index, function) in days.iter().enumerate() {
        println!("### Day {} ###\n", index + 1);

        function();

        println!();
    }
}

fn day01() {
    let inputs: Vec<_> = crate::files::read_file_line_per_line("inputs/day01.txt")
        .into_iter()
        .map(|string| string.parse::<i32>().unwrap())
        .collect();

    'outer_for_two: for a in 0..inputs.len() {
        for b in 1..inputs.len() {
            if (a != b) && (inputs[a] + inputs[b] == 2020) {
                let first = inputs[a];
                let second = inputs[b];

                println!(
                    "Found {} and {}, multiplied is {}",
                    first,
                    second,
                    first * second
                );
                break 'outer_for_two;
            }
        }
    }

    'outer_for_three: for a in 0..inputs.len() {
        for b in 1..inputs.len() {
            for c in 2..inputs.len() {
                if (a != b) && (b != c) && (inputs[a] + inputs[b] + inputs[c] == 2020) {
                    let first = inputs[a];
                    let second = inputs[b];
                    let third = inputs[c];

                    println!(
                        "Found {}, {} and {}, multiplied is {}",
                        first,
                        second,
                        third,
                        first * second * third
                    );
                    break 'outer_for_three;
                }
            }
        }
    }
}
