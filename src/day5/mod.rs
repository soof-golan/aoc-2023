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
    Ok(Solution {
        part1: "".to_string(),
        part2: "".to_string(),
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_numbers() {}
}
