use std::collections::{HashMap, HashSet};

use nom::branch::alt;
use nom::character::complete::u32 as u32_parser;
use nom::character::streaming::anychar;
use nom::combinator::map;
use nom::IResult;

#[derive(Debug, PartialEq, Eq, Hash, Copy, Clone)]
struct P {
    x: isize,
    y: isize,
}

#[derive(Debug, PartialEq, Eq, Hash)]
struct Number {
    x: isize,
    y: isize,
    value: u32,
}

fn maybe_number(input: &str) -> IResult<&str, Option<u32>> {
    alt((map(u32_parser, Some), map(anychar, |_| None)))(input)
}

pub fn run(input: &str) -> anyhow::Result<String> {
    let numbers = find_numbers(input);
    let mut maybe_gears = build_starmap(input);
    assert!(!numbers.is_empty());
    numbers
        .iter()
        .for_each(|n| update_neighbors(n.x, n.y, &mut maybe_gears, n.value));

    let sum = maybe_gears
        .iter()
        .filter(|(_, gear)| gear.num_neighbors == 2)
        .map(|(_, gear)| gear.value)
        .sum::<usize>();

    Ok(sum.to_string())
}

fn find_numbers(input: &str) -> Vec<Number> {
    let mut numbers = Vec::new();
    input.lines().enumerate().for_each(|(y, line)| {
        let mut input = line;
        loop {
            if input.is_empty() {
                break;
            }
            match maybe_number(input) {
                Ok((forward, maybe_n)) => {
                    input = forward;
                    if let Some(n) = maybe_n {
                        let x = line.len() - forward.len() - n.to_string().len();
                        numbers.push(Number {
                            x: x as isize,
                            y: y as isize,
                            value: n,
                        });
                    }
                }
                _ => continue,
            };
        }
    });
    numbers
}
fn build_starmap(input: &str) -> HashMap<P, Gear> {
    let mut symbols = HashMap::new();
    input.lines().enumerate().for_each(|(y, line)| {
        line.chars().enumerate().for_each(|(x, c)| {
            if let '*' = c {
                symbols.insert(
                    P {
                        x: x as isize,
                        y: y as isize,
                    },
                    Gear {
                        num_neighbors: 0,
                        value: 1,
                    },
                );
            }
        })
    });
    symbols
}

#[derive(Debug, PartialEq, Eq, Hash)]
struct Gear {
    num_neighbors: usize,
    value: usize,
}

fn update_neighbors(x: isize, y: isize, gears: &mut HashMap<P, Gear>, n: u32) {
    let moves: Vec<(isize, isize)> = vec![
        (-1, -1),
        (-1, 0),
        (-1, 1),
        (0, -1),
        (0, 0),
        (0, 1),
        (1, -1),
        (1, 0),
        (1, 1),
    ];
    let mut lookups: HashSet<P> = HashSet::new();
    n.to_string().chars().enumerate().for_each(|(i, _)| {
        moves.iter().for_each(|(dx, dy)| {
            let x = x + i as isize + dx;
            let y = y + dy;
            lookups.insert(P { x, y });
        })
    });

    assert!(lookups.len() >= 9);
    lookups.iter().for_each(|&p| {
        gears.entry(p).and_modify(|e| {
            e.num_neighbors += 1;
            e.value *= n as usize;
        });
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_find_numbers() {
        let input = "\
12.$
..3.
45*.\
"
        .to_string();
        let numbers = find_numbers(&input);
        assert_eq!(numbers.len(), 3);
        assert_eq!(
            numbers,
            vec![
                Number {
                    x: 0,
                    y: 0,
                    value: 12
                },
                Number {
                    x: 2,
                    y: 1,
                    value: 3
                },
                Number {
                    x: 0,
                    y: 2,
                    value: 45
                }
            ]
        );
    }

    #[test]
    fn test_build_gears() {
        let input = "\
12.$
..3.
45*.\
"
        .to_string();
        let stars = build_starmap(&input);
        assert_eq!(stars.len(), 1);
        assert!(stars.contains_key(&P { x: 2, y: 2 }));
    }

    #[test]
    fn test_gears_sum() {
        assert_eq!(
            run("467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..")
            .unwrap(),
            "467835"
        )
    }
}
