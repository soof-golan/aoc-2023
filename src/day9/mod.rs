use anyhow;
use itertools::Itertools;
use nom::bytes::complete::tag;
use nom::character::complete::{i32 as i32p, newline};
use nom::multi::separated_list1;
use nom::IResult;
use num::Integer;

use crate::solution::Solution;

type Reading = i32;
type Readings = Vec<Reading>;

pub fn run(input: &str) -> anyhow::Result<Solution> {
    let (_, histories) = histories(input).expect("failed to parse");
    Ok(Solution {
        part1: part1(&histories).to_string(),
        part2: part2(&histories).to_string(),
    })
}

fn numbers(input: &str) -> IResult<&str, Readings> {
    separated_list1(tag(" "), i32p)(input)
}

fn histories(input: &str) -> IResult<&str, Vec<Readings>> {
    separated_list1(newline, numbers)(input)
}

fn part1(histories: &Vec<Readings>) -> Reading {
    histories
        .iter()
        .fold(0, |acc, history| acc + predict_next(&history))
}
fn part2(histories: &Vec<Readings>) -> Reading {
    histories.iter().fold(0, |acc, history| {
        acc + predict_next(&history.iter().rev().map(|x| *x).collect())
    })
}

fn predict_next(history: &Readings) -> Reading {
    let diff: Readings = history.iter().tuple_windows().map(|(a, b)| b - a).collect();
    if diff.iter().all(|x| *x == 0) {
        history.last().unwrap().clone()
    } else {
        history.last().unwrap().clone() + predict_next(&diff)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_predict_next() {
        let input = vec![0, 3, 6];
        let expected = 9;
        assert_eq!(predict_next(&input), expected);
        let input = vec![0, 3, 6, 9, 12, 15];
        let expected = 18;
        assert_eq!(predict_next(&input), expected);
        // 10 13 16 21 30 45
        let input = vec![10, 13, 16, 21, 30, 45];
        let expected = 68;
        assert_eq!(predict_next(&input), expected);
    }

    #[test]
    fn test_numbers() {
        let input = "0 3 6 9 12 15";
        let expected = vec![0, 3, 6, 9, 12, 15];
        assert_eq!(numbers(input).unwrap().1, expected);
    }

    #[test]
    fn test_histories() {
        let input = "0 3 6 9 12 15
1 3 6 10 15 21
10 13 16 21 30 45";
        let expected = vec![
            vec![0, 3, 6, 9, 12, 15],
            vec![1, 3, 6, 10, 15, 21],
            vec![10, 13, 16, 21, 30, 45],
        ];
        assert_eq!(histories(input).unwrap().1, expected);
    }

    #[test]
    fn test_part1() {
        let input = "0 3 6 9 12 15
1 3 6 10 15 21
10 13 16 21 30 45";
        assert_eq!(run(input).unwrap().part1, "114");
    }
    #[test]
    fn test_part2() {
        let input = "0 3 6 9 12 15
1 3 6 10 15 21
10 13 16 21 30 45";
        assert_eq!(run(input).unwrap().part2, "2");
    }
}
