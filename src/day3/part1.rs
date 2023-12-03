use std::collections::HashSet;

use nom::branch::alt;
use nom::character::complete::u32 as u32_parser;
use nom::character::is_digit;
use nom::character::streaming::anychar;
use nom::combinator::map;
use nom::IResult;

#[derive(Debug, PartialEq, Eq, Hash)]
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
    alt((map(u32_parser, |n: u32| Some(n)), map(anychar, |_| None)))(input)
}

pub fn run(input: &str) -> anyhow::Result<String> {
    let symbols = build_symbols(&input);
    let numbers = find_numbers(input);
    let sum: usize = numbers
        .iter()
        .filter_map(|n| {
            if has_neighbors(n.x, n.y, &symbols, n.value) {
                Some(n.value as usize)
            } else {
                None
            }
        })
        .sum();

    Ok(sum.to_string())
}

fn find_numbers(input: &str) -> Vec<Number> {
    let mut numbers = Vec::new();
    let _ = input.lines().enumerate().for_each(|(y, line)| {
        let mut input = line;
        loop {
            if input.is_empty() {
                break;
            }
            match maybe_number(&input) {
                Ok((forward, maybe_n)) => {
                    input = forward;

                    match maybe_n {
                        Some(n) => {
                            let x = line.len() - forward.len() - n.to_string().len();
                            numbers.push(Number {
                                x: x as isize,
                                y: y as isize,
                                value: n,
                            });
                        }
                        None => {}
                    }
                }
                _ => continue,
            };
        }
    });
    numbers
}

fn build_symbols(input: &str) -> HashSet<P> {
    let mut symbols: HashSet<P> = HashSet::new();
    input.lines().enumerate().for_each(|(y, line)| {
        line.chars().enumerate().for_each(|(x, c)| match c {
            '.' => {}
            '0' | '1' | '2' | '3' | '4' | '5' | '6' | '7' | '8' | '9' => {}
            _ => {
                symbols.insert(P {
                    x: x as isize,
                    y: y as isize,
                });
            }
        })
    });
    symbols
}

fn has_neighbors(x: isize, y: isize, symbols: &HashSet<P>, n: u32) -> bool {
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
    let mut lookups: HashSet<(isize, isize)> = HashSet::new();
    n.to_string().chars().enumerate().for_each(|(i, _)| {
        moves.iter().for_each(|(dx, dy)| {
            let x = x + i as isize + dx;
            let y = y + dy;
            lookups.insert((x, y));
        })
    });

    assert!(lookups.len() >= 9);

    lookups
        .iter()
        .any(|(x, y)| symbols.contains(&P { x: *x, y: *y }))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_build_symbols() {
        let input = "\
12.$
..3.
45*.\
"
        .to_string();
        let symbols = build_symbols(&input);
        assert_eq!(symbols.len(), 2);
        assert!(symbols.contains(&P { x: 3, y: 0 }));
        assert!(symbols.contains(&P { x: 2, y: 2 }));
    }

    #[test]
    fn test_has_neighbors() {
        let input = "\
12.$
..3.
45*.\
"
        .to_string();
        let symbols = build_symbols(&input);
        assert!(!has_neighbors(0, 0, &symbols, 12));
        assert!(has_neighbors(2, 1, &symbols, 3));
        assert!(has_neighbors(0, 2, &symbols, 45));
    }

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
    fn test_schematics() {
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
            "4361"
        )
    }
}
