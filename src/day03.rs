pub fn solve() {
    let inputs = crate::files::read_file_line_per_line("inputs/day03.txt");

    let tree_count = count_trees(&inputs, 3, 1);

    println!("We bumped into {} trees", tree_count);

    let slopes = vec![(1, 1), (3, 1), (5, 1), (7, 1), (1, 2)];

    let product_of_trees: i64 = slopes
        .into_iter()
        .map(|(right_step, down_step)| count_trees(&inputs, right_step, down_step))
        .product();

    println!(
        "When taking all the slopes and multiply, we get {}",
        product_of_trees
    );
}

fn count_trees(inputs: &Vec<String>, right_step: usize, down_step: usize) -> i64 {
    let width = inputs[0].len();

    let mut current_line = 0;
    let mut current_index = 0;
    let mut tree_count = 0;

    while current_line < inputs.len() {
        let input = &inputs[current_line];
        if input.chars().nth(current_index).unwrap_or(' ') == '#' {
            tree_count += 1;
        }

        current_line += down_step;
        current_index = (current_index + right_step) % width;
    }

    tree_count
}
