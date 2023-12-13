use std::{
    sync::{
        atomic::{AtomicU32, Ordering},
        Arc,
    },
    thread,
};

use crossbeam_queue::SegQueue;
use rayon::iter::{IntoParallelRefIterator, ParallelIterator};

#[derive(Debug, Clone)]
#[allow(dead_code)] // I want to keep index as part of the card struct!
struct Card {
    index: u32,
    winning_numbers: Vec<u32>,
    present_numbers: Vec<u32>,
}

impl Card {
    pub fn winning_matches(&self) -> u32 {
        let mut count = 0;

        for num in self.present_numbers.iter() {
            if self.winning_numbers.contains(num) {
                count += 1;
            }
        }

        count
    }
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
        .map(|card| card.winning_matches())
        .map(get_score_from_winning_matches)
        .sum();

    println!("\tPart one: {sum}");
}

// Parallel version of the brute-force solution that I spent a loooong time on. It's useless, but I'm proud of it.
fn _part_two(input: &str) {
    let all_cards: Vec<Card> = input.lines().map(parse_card_from_line).collect();

    // create and populate queue
    let cards_processing_queue = Arc::new(SegQueue::new());
    all_cards
        .par_iter()
        .for_each(|card| cards_processing_queue.push(card.clone()));

    let all_cards = Arc::new(all_cards);

    // This is the count of all cards. I originally stored all the cards in a Vec, but we only care about the length
    // of the vec, not its contents, so this is a bit faster
    let card_count = Arc::new(AtomicU32::new(0));

    let threads: Vec<_> = (0..num_cpus::get())
        .map(|_| {
            let all_cards = all_cards.clone();
            let cards_processing_queue = cards_processing_queue.clone();
            let card_count = card_count.clone();

            thread::spawn(move || {
                while let Some(card) = cards_processing_queue.pop() {
                    let winning_matches = card.winning_matches();

                    // No card duplication, we can just move on
                    if winning_matches == 0 {
                        card_count.fetch_add(1, Ordering::Relaxed);
                        continue;
                    }

                    // The range of cards to duplicate
                    let lower_bound = card.index as usize;
                    let upper_bound = (card.index + winning_matches) as usize;

                    // Add all the duplicated cards back into the queue for their own processing
                    for i in lower_bound..upper_bound {
                        if let Some(card) = all_cards.get(i) {
                            cards_processing_queue.push(card.clone()); // card needs to be cloned
                        }
                    }

                    card_count.fetch_add(1, Ordering::Relaxed);
                }
            })
        })
        .collect();

    for handle in threads {
        handle.join().unwrap();
    }

    println!("\tPart two: {}", card_count.load(Ordering::Relaxed));
}

fn part_two(input: &str) {
    let mut card_counts: Vec<(Card, usize)> = input
        .lines()
        .map(parse_card_from_line)
        .zip(std::iter::repeat(1))
        .collect();

    let mut total_card_count = 0;

    for i in 0..card_counts.len() {
        let (current_card, current_count) = &card_counts[i];
        let current_count = *current_count;

        total_card_count += current_count;

        let winning_matches = current_card.winning_matches() as usize;

        for (_, count) in card_counts.iter_mut().skip(i + 1).take(winning_matches) {
            *count += current_count;
        }
    }

    println!("\tPart two: {}", total_card_count);
}

pub fn run() {
    println!("Day four:");

    let input = include_str!("input.txt");

    let instant = std::time::Instant::now();

    part_one(input);
    part_two(input);

    println!("\tTime: {} ms", instant.elapsed().as_millis());
}
