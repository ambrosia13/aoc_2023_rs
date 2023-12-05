use regex::Regex;

#[derive(Debug)]
struct CubeCounts {
    red_count: usize,
    green_count: usize,
    blue_count: usize,
}

impl CubeCounts {
    pub fn power(&self) -> usize {
        self.red_count * self.green_count * self.blue_count
    }
}

#[derive(Debug)]
struct Game {
    id: usize,
    rounds: Vec<CubeCounts>,
}

fn is_game_valid(game: &Game, cubes_available: &CubeCounts) -> bool {
    game.rounds
        .iter()
        .map(|round| {
            round.red_count <= cubes_available.red_count
                && round.green_count <= cubes_available.green_count
                && round.blue_count <= cubes_available.blue_count
        })
        .all(|b| b)
}

fn get_game_from_line(line: &str, patterns: (&Regex, &Regex, &Regex, &Regex)) -> Game {
    let (game_regex, red_regex, green_regex, blue_regex) = patterns;

    if let Some(caps) = game_regex.captures(line) {
        let rounds = caps["other_contents"]
            .split(';')
            .map(|entry| {
                let mut red_count = 0;
                let mut green_count = 0;
                let mut blue_count = 0;

                if let Some(caps) = red_regex.captures(entry) {
                    red_count = caps["count"].parse().unwrap();
                }

                if let Some(caps) = green_regex.captures(entry) {
                    green_count = caps["count"].parse().unwrap();
                }

                if let Some(caps) = blue_regex.captures(entry) {
                    blue_count = caps["count"].parse().unwrap();
                }

                CubeCounts {
                    red_count,
                    green_count,
                    blue_count,
                }
            })
            .collect();

        Game {
            id: caps["game_id"].parse().unwrap(),
            rounds,
        }
    } else {
        panic!()
    }
}

fn part_one(input: &str) {
    let cubes_available = CubeCounts {
        red_count: 12,
        green_count: 13,
        blue_count: 14,
    };

    let game_regex = Regex::new(r"Game (?P<game_id>\d+)\: (?P<other_contents>.+)").unwrap();

    let pattern = r"(?P<count>\d+)";

    let red_regex = Regex::new(&format!("{pattern} red")).unwrap();
    let green_regex = Regex::new(&format!("{pattern} green")).unwrap();
    let blue_regex = Regex::new(&format!("{pattern} blue")).unwrap();

    let sum: usize = input
        .trim()
        .lines()
        .map(|line| get_game_from_line(line, (&game_regex, &red_regex, &green_regex, &blue_regex)))
        .filter(|game| is_game_valid(game, &cubes_available))
        .map(|game| game.id)
        .sum();

    println!("\tPart one: {sum}");
}

fn find_required_cube_counts(game: &Game) -> CubeCounts {
    let mut max_red = 0;
    let mut max_green = 0;
    let mut max_blue = 0;

    game.rounds.iter().for_each(|round| {
        max_red = max_red.max(round.red_count);
        max_green = max_green.max(round.green_count);
        max_blue = max_blue.max(round.blue_count);
    });

    CubeCounts {
        red_count: max_red,
        green_count: max_green,
        blue_count: max_blue,
    }
}

fn part_two(input: &str) {
    let game_regex = Regex::new(r"Game (?P<game_id>\d+)\: (?P<other_contents>.+)").unwrap();

    let pattern = r"(?P<count>\d+)";

    let red_regex = Regex::new(&format!("{pattern} red")).unwrap();
    let green_regex = Regex::new(&format!("{pattern} green")).unwrap();
    let blue_regex = Regex::new(&format!("{pattern} blue")).unwrap();

    let sum: usize = input
        .trim()
        .lines()
        .map(|line| get_game_from_line(line, (&game_regex, &red_regex, &green_regex, &blue_regex)))
        .map(|game| find_required_cube_counts(&game))
        .map(|cube_counts| cube_counts.power())
        .sum();

    println!("\tPart two: {sum}");
}

pub fn run() {
    println!("Day two:");

    let input = include_str!("input.txt");

    let instant = std::time::Instant::now();

    part_one(input);
    part_two(input);

    println!("\tTime: {} ms", instant.elapsed().as_millis());
}
