use std::collections::HashMap;

use nom::bytes::complete::{tag, take};
use nom::character::complete::{newline, u32};
use nom::combinator::map;
use nom::multi::separated_list1;
use nom::sequence::separated_pair;
use nom::IResult;

use crate::solution::Solution;

pub fn run(input: &str) -> anyhow::Result<Solution> {
    let mut hands = hands(input).expect("failed to parse hands").1;
    let _ = &hands.sort_by(|a, b| {
        let a_hand_type = HandType::part1(&a.cards);
        let b_hand_type = HandType::part1(&b.cards);
        let a_hand = a.cards.iter().map(char_to_card).collect::<Vec<Card>>();
        let b_hand = b.cards.iter().map(char_to_card).collect::<Vec<Card>>();
        (a_hand_type, a_hand).cmp(&(b_hand_type, b_hand))
    });
    let part1 = hands
        .iter()
        .enumerate()
        .fold(0, |acc, (i, hand)| acc + hand.bid * (i + 1) as u32);

    let _ = &hands.sort_by(|a, b| {
        let a_hand_type = HandType::part2(&a.cards);
        let b_hand_type = HandType::part2(&b.cards);
        let a_hand = a
            .cards
            .iter()
            .map(char_to_card_part2)
            .collect::<Vec<Card>>();
        let b_hand = b
            .cards
            .iter()
            .map(char_to_card_part2)
            .collect::<Vec<Card>>();
        (a_hand_type, a_hand).cmp(&(b_hand_type, b_hand))
    });

    let part2 = hands
        .iter()
        .enumerate()
        .fold(0, |acc, (i, hand)| acc + hand.bid * (i + 1) as u32);

    Ok(Solution {
        part1: part1.to_string(),
        part2: part2.to_string(),
    })
}

fn hand(input: &str) -> IResult<&str, Hand> {
    map(
        separated_pair(
            map(take(5usize), |s: &str| s.bytes().collect::<Vec<u8>>()),
            tag(" "),
            u32,
        ),
        |(cards, bid)| Hand::new(cards, bid),
    )(input)
}

fn hands(input: &str) -> IResult<&str, Vec<Hand>> {
    separated_list1(newline, hand)(input)
}

type Card = u8;

#[derive(Debug, Clone, Copy, Ord, PartialOrd, Eq, PartialEq)]
enum HandType {
    HighCard = 0,
    OnePair = 1,
    TwoPair = 2,
    ThreeOfAKind = 3,
    FullHouse = 4,
    FourOfAKind = 5,
    FiveOfAKind = 6,
}

impl HandType {
    fn part1(cards: &Vec<Card>) -> HandType {
        let freq_map = into_character_map(cards.to_vec());
        assert_eq!(freq_map.values().sum::<u32>(), 5);
        let mut sorted_values = freq_map.values().collect::<Vec<&u32>>();
        sorted_values.sort();
        match sorted_values.as_slice() {
            [5] => HandType::FiveOfAKind,
            [1, 4] => HandType::FourOfAKind,
            [2, 3] => HandType::FullHouse,
            [1, 1, 3] => HandType::ThreeOfAKind,
            [1, 2, 2] => HandType::TwoPair,
            [1, 1, 1, 2] => HandType::OnePair,
            [1, 1, 1, 1, 1] => HandType::HighCard,
            _ => panic!("Invalid hand"),
        }
    }

    fn part2(cards: &Vec<Card>) -> HandType {
        let mut freq_map = into_character_map(cards.to_vec());
        assert_eq!(freq_map.values().sum::<u32>(), 5);
        let jokers: u32 = freq_map.remove(&b'J').unwrap_or(0).clone();
        let mut sorted_values: Vec<u32> = freq_map.values().map(|v| *v).collect();
        sorted_values.sort();
        let len = sorted_values.len();
        let sorted_values = match len {
            0 => vec![jokers],
            _ => {
                sorted_values[len - 1] += jokers;
                sorted_values
            }
        };
        assert_eq!(sorted_values.iter().sum::<u32>(), 5);

        match sorted_values.as_slice() {
            [5] => HandType::FiveOfAKind,
            [1, 4] => HandType::FourOfAKind,
            [2, 3] => HandType::FullHouse,
            [1, 1, 3] => HandType::ThreeOfAKind,
            [1, 2, 2] => HandType::TwoPair,
            [1, 1, 1, 2] => HandType::OnePair,
            [1, 1, 1, 1, 1] => HandType::HighCard,
            _ => panic!("Invalid hand"),
        }
    }
}

#[derive(Debug, Clone, Ord, PartialOrd, Eq, PartialEq)]
struct Hand {
    cards: Vec<u8>,
    bid: u32,
}

fn into_character_map(chars: Vec<u8>) -> HashMap<u8, u32> {
    chars.iter().fold(HashMap::with_capacity(4), |mut acc, c| {
        *acc.entry(c.clone()).or_insert(0) += 1;
        acc
    })
}

#[inline]
fn char_to_card(u8: &u8) -> Card {
    match u8 {
        b'2'..=b'9' => u8 - b'0',
        b'T' => 10,
        b'J' => 11,
        b'Q' => 12,
        b'K' => 13,
        b'A' => 14,
        _ => panic!("Invalid card"),
    }
}

#[inline]
fn char_to_card_part2(u8: &u8) -> Card {
    match u8 {
        b'2'..=b'9' => u8 - b'0',
        b'T' => 10,
        b'J' => 1,
        b'Q' => 12,
        b'K' => 13,
        b'A' => 14,
        _ => panic!("Invalid card"),
    }
}

impl Hand {
    fn new(cards: Vec<u8>, bid: u32) -> Hand {
        assert_eq!(cards.len(), 5);
        Hand { cards, bid }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_freq_map() {
        let expected: HashMap<u8, u32> = HashMap::from_iter(vec![(b'A', 5)]);
        assert_eq!(
            into_character_map(vec![b'A', b'A', b'A', b'A', b'A']),
            expected
        );
        let expected: HashMap<u8, u32> = HashMap::from_iter(vec![(b'A', 4), (b'B', 1)]);
        assert_eq!(
            into_character_map(vec![b'A', b'A', b'A', b'A', b'B']),
            expected
        );
    }

    #[test]
    fn test_hand_parser() {
        let input = "32T3K 765";
        assert_eq!(
            hand(input).unwrap().1,
            Hand {
                cards: vec![3, 2, 10, 3, 13],
                bid: 765
            }
        );
    }
    #[test]
    fn test_hands_parser() {
        let input = "32T3K 765
T55J5 684";
        assert_eq!(
            hands(input).unwrap().1,
            vec![
                Hand {
                    cards: vec![3, 2, 10, 3, 13],
                    bid: 765
                },
                Hand {
                    cards: vec![10, 5, 5, 11, 5],
                    bid: 684
                }
            ]
        );
    }

    #[test]
    fn test_part1() {
        let input = "32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483";
        assert_eq!(run(input).unwrap().part1, "6440");
    }
    #[test]
    fn test_part2() {
        let input = "32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483";
        assert_eq!(run(input).unwrap().part2, "5905");
    }
}
