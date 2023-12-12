use crossbeam_queue::SegQueue;

#[derive(Debug, Clone)]
struct Card {
    index: u32,
    winning_numbers: Vec<u32>,
    present_numbers: Vec<u32>,
}

fn parse_card_from_line(line: &str) -> Card {
    let mut tokens = line.trim().split_ascii_whitespace();

    let index = tokens
        .nth(1)
        .unwrap()
        .replace(':', "") // remove the colon
        .parse::<u32>()
        .unwrap();

    let mut winning_numbers = Vec::new();
    let mut present_numbers = Vec::new();

    let mut parsing_winning_numbers = true;

    for token in tokens {
        if token == "|" {
            parsing_winning_numbers = false;
            continue;
        }

        let number = token.parse::<u32>().unwrap();

        if parsing_winning_numbers {
            winning_numbers.push(number);
        } else {
            present_numbers.push(number);
        }
    }

    Card {
        index,
        winning_numbers,
        present_numbers,
    }
}

fn count_winning_matches_in_card(card: &Card) -> u32 {
    let mut count = 0;

    for num in card.present_numbers.iter() {
        if card.winning_numbers.contains(num) {
            count += 1;
        }
    }

    count
}

fn get_score_from_winning_matches(winning_matches: u32) -> u32 {
    if winning_matches > 0 {
        1 << (winning_matches - 1)
    } else {
        0
    }
}

fn part_one(input: &str) {
    let sum: u32 = input
        .lines()
        .map(parse_card_from_line)
        .map(|card| count_winning_matches_in_card(&card))
        .map(get_score_from_winning_matches)
        .sum();

    println!("\tPart one: {sum}");
}

fn part_two(input: &str) {
    let all_cards: Vec<Card> = input.lines().map(parse_card_from_line).collect();

    // create and populate queue
    let cards_processing_queue = SegQueue::new();
    for card in input.lines().map(parse_card_from_line) {
        cards_processing_queue.push(card);
    }

    let mut processed_cards = Vec::new();

    while let Some(card) = cards_processing_queue.pop() {
        let winning_matches = count_winning_matches_in_card(&card);

        // Start from the index+1, and since indices are already offset, just use the card's index
        let lower_bound = card.index as usize;
        let upper_bound = (card.index + winning_matches) as usize;

        // the cards that we will duplicate
        for i in lower_bound..upper_bound {
            if let Some(card) = all_cards.get(i) {
                cards_processing_queue.push(card.clone()); // clone the card and push it to the processing queue
            }
        }

        // now we're done with the current card from the queue, add it to processed_cards
        processed_cards.push(card);
    }

    println!("\tPart two: {}", processed_cards.len());
}

pub fn run() {
    println!("Day four:");

    let input = include_str!("input.txt");

    let instant = std::time::Instant::now();

    part_one(input);
    part_two(input);

    println!("\tTime: {} ms", instant.elapsed().as_millis());
}
