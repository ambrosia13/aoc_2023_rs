use array2d::Array2D;

#[derive(Debug)]
struct NumberEntry<'a> {
    number: u32,
    neighbors: &'a [char],
}

impl<'a> NumberEntry<'a> {
    fn is_engine_part(&self) -> bool {
        self.neighbors.iter().any(|&ch| ch != '.')
    }
}

fn part_one(input: &str) {}
fn part_two(input: &str) {}

pub fn run() {
    println!("Day three:");

    let input = include_str!("input.txt");

    let instant = std::time::Instant::now();

    part_one(input);
    part_two(input);

    println!("\tTime: {} ms", instant.elapsed().as_millis());
}
