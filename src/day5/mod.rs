
use std::usize;

use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::character::complete::newline;
use nom::character::complete::u32 as u32_parser;
use nom::combinator::{map, opt};
use nom::multi::{fold_many1, many1};
use nom::sequence::{separated_pair, terminated, tuple};
use nom::IResult;

use crate::solution::Solution;

pub fn run(input: &str) -> anyhow::Result<Solution> {
    let cards = collect_cards(input);
    let counts: Vec<usize> = cards.iter().map(ScratchCardNumbers::count).collect();
    let part1_sum = counts.iter().map(score).sum::<usize>();

    let mut card_count: Vec<usize> = counts.iter().map(|_| 1).collect();
    for (id, probe) in counts.iter().enumerate() {
        let our_count = *card_count.get(id).unwrap();
        let probe_start = id + 1;
        let probe_end = probe_start + probe;
        for i in probe_start..probe_end {
            match card_count.get_mut(i) {
                Some(count) => *count += our_count,
                None => (),
            }
        }
    }
    let part2_sum = card_count.iter().sum::<usize>();

    Ok(Solution {
        part1: part1_sum.to_string(),
        part2: part2_sum.to_string(),
    })
}

fn collect_cards(input: &str) -> Vec<ScratchCardNumbers> {
    let cards: Vec<ScratchCardNumbers> = input
        .lines()
        .filter_map(|line| match parse_card(line) {
            Ok((_, card)) => Some(card),
            Err(_) => None,
        })
        .map(|card| card.numbers)
        .collect();
    cards
}

#[derive(Debug, PartialEq, Eq)]
struct Card {
    id: u32,
    numbers: ScratchCardNumbers,
}

#[derive(Debug, PartialEq, Eq)]
struct ScratchCardNumbers {
    winning_numbers: Vec<u32>,
    numbers: Vec<u32>,
}
fn score(count: &usize) -> usize {
    match count {
        0 => 0,
        _ => 1usize << (count - 1),
    }
}

impl ScratchCardNumbers {
    fn count(&self) -> usize {
        self.numbers
            .iter()
            .filter(|n| self.winning_numbers.contains(n))
            .count()
    }
}

fn maybe_number(input: &str) -> IResult<&str, Option<u32>> {
    alt((map(u32_parser, Some), map(tag(" "), |_| None)))(input)
}

fn numbers(input: &str) -> IResult<&str, Vec<u32>> {
    fold_many1(
        maybe_number,
        || {
            let mut v = Vec::new();
            v.reserve(24);
            v
        },
        |mut acc, item| {
            if let Some(n) = item {
                acc.push(n);
            };
            acc
        },
    )(input)
}

fn parse_scratchcard_numbers(input: &str) -> IResult<&str, ScratchCardNumbers> {
    map(separated_pair(numbers, tag("|"), numbers), |item| {
        ScratchCardNumbers {
            winning_numbers: item.0,
            numbers: item.1,
        }
    })(input)
}

fn parse_card(input: &str) -> IResult<&str, Card> {
    map(
        terminated(
            tuple((
                tag("Card"),
                many1(tag(" ")),
                u32_parser,
                tag(":"),
                parse_scratchcard_numbers,
            )),
            opt(newline),
        ),
        |(_, _, id, _, scn)| Card { id, numbers: scn },
    )(input)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_numbers() {
        assert_eq!(
            parse_card("Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53")
                .unwrap()
                .1,
            Card {
                id: 1,
                numbers: ScratchCardNumbers {
                    winning_numbers: vec![41, 48, 83, 86, 17],
                    numbers: vec![83, 86, 6, 31, 17, 9, 48, 53],
                }
            }
        );
    }
    #[test]
    fn test_parse_card() {
        assert_eq!(
            parse_scratchcard_numbers("41 48 83 86 17 | 83 86  6 31 17  9 48 53")
                .unwrap()
                .1,
            ScratchCardNumbers {
                winning_numbers: vec![41, 48, 83, 86, 17],
                numbers: vec![83, 86, 6, 31, 17, 9, 48, 53],
            }
        );
    }
    #[test]
    fn test_score() {
        assert_eq!(
            score(
                &ScratchCardNumbers {
                    winning_numbers: vec![41, 48, 83, 86, 17],
                    numbers: vec![83, 86, 6, 31, 17, 9, 48, 53],
                }
                .count()
            ),
            8
        );
    }

    #[test]
    fn test_collect_cards() {
        let input = "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11";
        assert_eq!(
            collect_cards(input),
            vec![
                ScratchCardNumbers {
                    winning_numbers: vec![41, 48, 83, 86, 17],
                    numbers: vec![83, 86, 6, 31, 17, 9, 48, 53],
                },
                ScratchCardNumbers {
                    winning_numbers: vec![13, 32, 20, 16, 61],
                    numbers: vec![61, 30, 68, 82, 17, 32, 24, 19],
                },
                ScratchCardNumbers {
                    winning_numbers: vec![1, 21, 53, 59, 44],
                    numbers: vec![69, 82, 63, 72, 16, 21, 14, 1],
                },
                ScratchCardNumbers {
                    winning_numbers: vec![41, 92, 73, 84, 69],
                    numbers: vec![59, 84, 76, 51, 58, 5, 54, 83],
                },
                ScratchCardNumbers {
                    winning_numbers: vec![87, 83, 26, 28, 32],
                    numbers: vec![88, 30, 70, 12, 93, 22, 82, 36],
                },
                ScratchCardNumbers {
                    winning_numbers: vec![31, 18, 13, 56, 72],
                    numbers: vec![74, 77, 10, 23, 35, 67, 36, 11],
                },
            ]
        );
    }

    #[test]
    fn test_part1() {
        let input = "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11";
        assert_eq!(run(input).unwrap().part1, "13");
    }

    #[test]
    fn test_part2() {
        let input = "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11";
        assert_eq!(run(input).unwrap().part2, "30");
    }
}
