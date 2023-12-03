use std::cmp::max;
use std::ops::{Add, AddAssign};

use nom::bytes::complete::tag;
use nom::character::complete::{alpha1, digit1};
use nom::combinator::{map_res, opt};
use nom::IResult;
use nom::multi::fold_many0;

#[derive(Debug, Eq, PartialEq, Ord, PartialOrd)]
struct CubeCount {
    r: u32,
    g: u32,
    b: u32,
}

impl Add<CubeCount> for CubeCount {
    type Output = CubeCount;

    fn add(self, rhs: CubeCount) -> Self::Output {
        CubeCount {
            r: self.r + rhs.r,
            g: self.g + rhs.g,
            b: self.b + rhs.b,
        }
    }
}

impl AddAssign<CubeCount> for CubeCount {
    fn add_assign(&mut self, rhs: CubeCount) {
        self.r += rhs.r;
        self.g += rhs.g;
        self.b += rhs.b;
    }
}

impl CubeCount {
    fn new() -> Self {
        CubeCount {
            r: 0,
            g: 0,
            b: 0,
        }
    }

    fn all_lt(&self, other: &CubeCount) -> bool {
        self.r <= other.r && self.g <= other.g && self.b <= other.b
    }

    fn bump(&mut self, other: &CubeCount) {
        self.r = max(self.r, other.r);
        self.g = max(self.g, other.g);
        self.b = max(self.b, other.b);
    }

    fn power(&self) -> u32 {
        self.r * self.g * self.b
    }


}


fn game_id(input: &str) -> IResult<&str, usize> {
    let (input, _) = tag("Game ")(input)?;
    let (input, game_id) = map_res(digit1, str::parse)(input)?;
    let (input, _) = tag(": ")(input)?;
    Ok((input, game_id))
}

fn str_to_count(input: &str, count: u32) -> CubeCount {
    match input {
        "red" => CubeCount { r: count, g: 0, b: 0 },
        "green" => CubeCount { r: 0, g: count, b: 0 },
        "blue" => CubeCount { r: 0, g: 0, b: count },
        _ => panic!("Unknown color: {:?}", input),
    }
}


/// ```rust
/// assert_eq!(parse_color("3 blue"), Ok(("", CubeCount { r: 0, g: 0, b: 3 })));
/// ```
///
fn parse_color(input: &str) -> IResult<&str, CubeCount> {
    let (input, count): (&str, u32) = map_res(digit1, str::parse)(input)?;
    let (input, _) = tag(" ")(input)?;
    let (input, color) = alpha1(input)?;
    let (input, _) = opt(tag(","))(input)?; // consume the comma
    let (input, _) = opt(tag(" "))(input)?; // consume the comma
    return Ok((input, str_to_count(color, count)));
}

fn parse_round(input: &str) -> IResult<&str, CubeCount> {
    if input.is_empty() {
        return Err(nom::Err::Error(nom::error::Error::new(input, nom::error::ErrorKind::Eof)));
    }
    let (input, _) = opt(tag(";"))(input)?; // consume the semicolon
    let (input, _) = opt(tag(" "))(input)?; // consume the semicolon
    let (input, result) = fold_many0(
        parse_color,
        CubeCount::new,
        |mut acc, item| {
            acc.bump(&item);
            acc
        },
    )(input)?;
    Ok((input, result))
}


/// pareses: '3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green, 1 green'
fn parse_game(input: &str) -> IResult<&str, CubeCount> {
    fold_many0(
        parse_round,
        CubeCount::new,
        |mut acc, item| {
            acc.bump(&item);
            acc
        },
    )(input)
}


pub fn run(input: &str) -> anyhow::Result<String> {
    let sum: usize = input.lines()
        .filter_map(|line| Some(game_id(line).ok()?))
        .filter_map(|(line, game_id)| {
            let (_, cubeset) = parse_game(line).ok()?;
            Some(cubeset.power() as usize)
        })
        // sum the cubes set power
        .sum();
    Ok(sum.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_color() {
        assert_eq!(parse_color("3 blue"), Ok(("", CubeCount { r: 0, g: 0, b: 3 })));
        assert_eq!(parse_color("4 red"), Ok(("", CubeCount { r: 4, g: 0, b: 0 })));
        assert_eq!(parse_color("2 green"), Ok(("", CubeCount { r: 0, g: 2, b: 0 })));
    }

    #[test]
    fn test_parse_round() {
        assert_eq!(parse_round("3 blue, 4 red, 1 blue").unwrap().1, CubeCount { r: 4, g: 0, b: 4 });
        assert_eq!(parse_round("3 blue, 4 red, 1 blue;").unwrap().1, CubeCount { r: 4, g: 0, b: 4 });
        assert_eq!(parse_round("3 blue, 4 red, 1 blue; 2 blue").unwrap(), ("; 2 blue", CubeCount { r: 4, g: 0, b: 4 }));
        assert_eq!(parse_round("; 2 blue").unwrap().1, CubeCount { r: 0, g: 0, b: 2 });
    }

    #[test]
    fn test_parse_game() {
        assert_eq!(parse_game("3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green;").unwrap().1, vec![
            CubeCount { r: 4, g: 0, b: 3 },
            CubeCount { r: 1, g: 2, b: 6 },
            CubeCount { r: 0, g: 2, b: 0 },
            CubeCount { r: 0, g: 0, b: 0 },
        ]);
        assert_eq!(parse_game("13 green, 1 red, 5 blue; 2 red, 5 green, 7 blue; 19 green, 5 blue; 4 blue, 13 green; 5 green, 8 blue").unwrap().1, vec![
            CubeCount { r: 1, g: 13, b: 5 },
            CubeCount { r: 2, g: 5, b: 7 },
            CubeCount { r: 0, g: 19, b: 5 },
            CubeCount { r: 0, g: 13, b: 4 },
            CubeCount { r: 0, g: 5, b: 8 },
        ]);

        let too_high =CubeCount { r: 0, g: 19, b: 5 };
        let limits = CubeCount { r: 12, g: 13, b: 14 };
        assert_eq!(too_high.all_lt(&limits), false);
    }

    #[test]
    fn test_part1() {
        assert_eq!(run(
            "
Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green
            "
        ).unwrap(), "8");

        assert_eq!(run(
            "
Game 98: 13 green, 1 red, 5 blue; 2 red, 5 green, 7 blue; 19 green, 5 blue; 4 blue, 13 green; 5 green, 8 blue
"
        ).unwrap(), "0");

        assert_eq!(run(
            "
Game 1: 1 blue; 4 green, 5 blue; 11 red, 3 blue, 11 green; 1 red, 10 green, 4 blue; 17 red, 12 green, 7 blue; 3 blue, 19 green, 15 red
Game 99: 11 red, 8 green; 16 red, 10 green; 9 red, 6 green; 3 blue, 2 red, 4 green
Game 100: 4 red, 2 blue, 4 green; 2 green, 1 red, 1 blue; 3 green, 4 blue, 5 red; 18 red, 2 blue; 9 red, 5 green, 4 blue
    "
        ).unwrap(), "0");
    }
}