use std::ops::Add;

fn collect_digits_in_line(input: &str) -> Vec<u32> {
    let mut digits = Vec::new();

    for ch in input.chars() {
        if ch.is_digit(10) {
            digits.push(ch.to_digit(10).unwrap());
        }
    }

    digits
}

fn sum_calibrations(input: &str) -> u32 {
    input
        .trim()
        .lines()
        .map(|line| collect_digits_in_line(line.trim()))
        .map(|digits| digits.first().unwrap() * 10 + digits.last().unwrap())
        .sum()
}

fn part_one(input: &str) {
    println!("\tPart one: {}", sum_calibrations(input));
}

fn convert_words_to_digits_in_line(input: &str) -> String {
    let numbers = [
        ("one", 1),
        ("two", 2),
        ("three", 3),
        ("four", 4),
        ("five", 5),
        ("six", 6),
        ("seven", 7),
        ("eight", 8),
        ("nine", 9),
    ];

    // to avoid modification headache, just make a new string
    let mut parsed_line = String::new();

    // I gave up on making this variable make sense
    let mut word_length_to_ignore = 0;

    for (index, ch) in input.char_indices() {
        numbers.iter().for_each(|&(word, digit)| {
            // lookahead from the current index until the current index plus the word length
            let lookahead = input.get(index..(index + word.len()));

            // make sure the substring is actually valid so we don't panic lol
            if let Some(lookahead) = lookahead {
                if lookahead == word {
                    // push the number as a digit to the string
                    parsed_line.push(char::from_digit(digit, 10).unwrap());
                    word_length_to_ignore = word.len();
                }
            }
        });

        if word_length_to_ignore == 0 {
            // If we didn't replace a number word thingy, then continue as normal and make no change
            parsed_line.push(ch);
        } else {
            word_length_to_ignore -= 1;
        }
    }

    parsed_line
}

fn part_two(input: &str) {
    let input = input
        .lines()
        .map(|line| convert_words_to_digits_in_line(line.trim()))
        .reduce(|mut accum, elem| {
            accum.push('\n');
            accum.add(&elem)
        })
        .expect("Couldn't convert numbers from words to digits on input");

    println!("\tPart two: {}", sum_calibrations(&input));
}

pub fn run() {
    println!("Day one:");

    let input = include_str!("input.txt");

    part_one(input);
    part_two(input);
}
