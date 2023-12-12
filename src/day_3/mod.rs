use array2d::Array2D;

#[derive(Debug)]
struct NumberEntry {
    number: u32,
    neighbors: Vec<char>,
}

impl NumberEntry {
    fn is_engine_part(&self) -> bool {
        self.neighbors.iter().any(|&ch| ch != '.')
    }
}

fn collect_array(input: &str) -> Array2D<char> {
    let input = input.trim();

    // This +1 magically fixes everything and I don't know why.
    let width = input.lines().next().unwrap().trim().len() + 1;
    let height = input.lines().count();

    let mut array = Array2D::filled_with('.', height, width);

    for (row, line) in input.lines().enumerate() {
        for (column, ch) in line.trim().char_indices() {
            array[(row, column)] = ch;
        }
    }

    array
}

fn get_number_entries(array: &Array2D<char>) -> Vec<NumberEntry> {
    let mut number_entries = Vec::new();

    for (row_index, row) in array.rows_iter().enumerate() {
        let mut digits_indices: Vec<(u32, (usize, usize))> = Vec::new();

        for (column_index, &ch) in row.enumerate() {
            if let Some(digit) = ch.to_digit(10) {
                digits_indices.push((digit, (row_index, column_index)));
            } else {
                let digit = digits_indices
                    .iter()
                    .map(|(digit, _)| *digit)
                    .reduce(|accum, elem| accum * 10 + elem);

                if let Some(digit) = digit {
                    let indices: Vec<(usize, usize)> =
                        digits_indices.iter().map(|(_, index)| *index).collect();

                    let offsets = [
                        (1, 1),
                        (-1, 1),
                        (1, -1),
                        (-1, -1),
                        (0, 1),
                        (0, -1),
                        (1, 0),
                        (-1, 0),
                    ];

                    let mut neighbors = Vec::new();

                    for index in indices {
                        let index = (index.0 as isize, index.1 as isize);

                        for offset in offsets {
                            let new_index = (index.0 + offset.0, index.1 + offset.1);

                            if let Some(neighbor) =
                                array.get(new_index.0 as usize, new_index.1 as usize)
                            {
                                if !neighbor.is_ascii_digit() {
                                    neighbors.push(*neighbor);
                                }
                            }
                        }
                    }

                    number_entries.push(NumberEntry {
                        number: digit,
                        neighbors,
                    });
                }

                digits_indices.clear();
            }
        }
    }

    number_entries
}

fn part_one(input: &str) {
    let array = collect_array(input);
    let number_entries = get_number_entries(&array);

    let sum: u32 = number_entries
        .iter()
        .filter(|entry| entry.is_engine_part())
        .map(|entry| entry.number)
        .sum();

    println!("\tPart one: {sum}");
}
fn part_two(_input: &str) {}

pub fn run() {
    println!("Day three:");

    let input = include_str!("input.txt");

    let instant = std::time::Instant::now();

    part_one(input);
    part_two(input);

    // println!("{:#?}", get_number_entries(&collect_array(input)));

    println!("\tTime: {} ms", instant.elapsed().as_millis());
}
