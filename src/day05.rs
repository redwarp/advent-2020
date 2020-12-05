struct BoardingPass {
    number: String,
}

impl BoardingPass {
    fn row_and_column(&self) -> (i32, i32) {
        let mut row_range = 0..128;
        let mut column_range = 0..8;

        for letter in self.number.chars() {
            match letter {
                'F' => {
                    row_range.end = row_range.end - (row_range.end - row_range.start) / 2;
                }
                'B' => {
                    row_range.start = row_range.start + (row_range.end - row_range.start) / 2;
                }
                'L' => {
                    column_range.end =
                        column_range.end - (column_range.end - column_range.start) / 2;
                }
                'R' => {
                    column_range.start =
                        column_range.start + (column_range.end - column_range.start) / 2;
                }
                _ => {}
            }
        }

        (row_range.start, column_range.start)
    }

    fn unique_id(&self) -> i32 {
        let (row, column) = self.row_and_column();
        return row * 8 + column;
    }

    fn index(&self) -> usize {
        let (row, column) = self.row_and_column();
        (row * 8 + column) as usize
    }
}

pub fn solve() {
    let pass = BoardingPass {
        number: String::from("BBFFBBFRLL"),
    };

    let place = pass.row_and_column();

    println!(
        "Test boarding pass: row {}, column {}, unique id {}",
        place.0,
        place.1,
        pass.unique_id()
    );

    let inputs = crate::files::read_file_line_per_line("inputs/day05.txt");
    let boarding_passes: Vec<_> = inputs
        .into_iter()
        .map(|number| BoardingPass { number })
        .collect();

    let highest_seat_id = boarding_passes
        .iter()
        .map(|pass| pass.unique_id())
        .max()
        .unwrap_or(0);
    println!("Highest seat id: {}", highest_seat_id);

    let mut filled_seats = vec![false; 128 * 8];
    boarding_passes.iter().for_each(|pass| {
        filled_seats[pass.index()] = true;
    });

    for index in 8..(127 * 8) {
        if !filled_seats[index] {
            let row = index % 8;
            let column = index / 8;
            // println!(
            //     "Seat at row {}, column {} is not filled. (Index {})",
            //     row,
            //     column,
            //     column * 8 + row
            // );

            // Well, there was too many of them to print, so just looked at the isolated one and it worked.
            // Sue me.
        }
    }
}
