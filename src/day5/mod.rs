use std::collections::BTreeMap;
use std::ops::Bound;

use nom::bytes::complete::tag;
use nom::character::complete::{alpha1, newline, u64};
use nom::combinator::{map, opt};
use nom::multi::{fold_many1, separated_list1};
use nom::sequence::{delimited, preceded, separated_pair, terminated, tuple};
use nom::IResult;

use crate::solution::Solution;

type BTM = BTreeMap<u64, i64>;

pub fn run(input: &str) -> anyhow::Result<Solution> {
    let (input, state) = parse_state(input).expect("failed to parse state");
    assert_eq!(input, "");
    let (seeds, translation_maps): (Vec<u64>, Vec<BTM>) = (state.seeds, state.translation_maps);

    let ranged: u64 = seeds
        .chunks_exact(2)
        .flat_map(|chunk| {
            let start = chunk[0] - 1;
            let end = chunk[0] + chunk[1];
            println!("{} - {}", start, end);
            (start..end).collect::<Vec<u64>>()
        })
        .map(|seed| {
            translation_maps
                .iter()
                .fold(seed, |seed, map| translate(&map, seed))
        })
        .min()
        .unwrap();

    let part1 = seeds
        // Hack start: Solves a compiler complaint that I couldn't figure out how to fix
        .chunks_exact(1)
        .flat_map(|chunk| chunk[0]..=chunk[0])
        // Hack end
        .map(|seed| {
            translation_maps
                .iter()
                .fold(seed, |seed, map| translate(&map, seed))
        })
        .min()
        .unwrap();

    Ok(Solution {
        part1: part1.to_string(),
        part2: ranged.to_string(),
    })
}

fn parse_numbers(input: &str) -> IResult<&str, Vec<u64>> {
    separated_list1(tag(" "), u64)(input)
}

fn seeds(input: &str) -> IResult<&str, Vec<u64>> {
    preceded(tag("seeds: "), parse_numbers)(input)
}

#[derive(Debug, PartialEq)]
struct Category {
    source: u64,
    range_length: u64,
    offset: i64,
}

/// returns (source, range_length, offset)
/// offset = destination - source
fn category(input: &str) -> IResult<&str, Category> {
    map(
        tuple((u64, tag(" "), u64, tag(" "), u64)),
        |(destination, _, source, _, range_length)| Category {
            source,
            range_length,
            offset: destination as i64 - source as i64,
        },
    )(input)
}

fn category_map(input: &str) -> IResult<&str, BTM> {
    preceded(
        tag("map:\n"),
        fold_many1(
            terminated(category, opt(newline)),
            BTreeMap::new,
            |mut acc, category| {
                acc.insert(category.source, category.offset);
                acc.insert(category.source + category.range_length - 1, 0);
                acc
            },
        ),
    )(input)
}

#[derive(Debug, PartialEq)]
struct MapNames {
    from: String,
    to: String,
}

fn map_names(input: &str) -> IResult<&str, MapNames> {
    map(
        delimited(
            opt(newline),
            separated_pair(alpha1, tag("-to-"), alpha1),
            tag(" "),
        ),
        |(from, to): (&str, &str)| MapNames {
            from: from.to_string(),
            to: to.to_string(),
        },
    )(input)
}

fn maps(input: &str) -> IResult<&str, Vec<BTM>> {
    separated_list1(newline, preceded(map_names, category_map))(input)
}

fn parse_state(input: &str) -> IResult<&str, State> {
    map(
        tuple((preceded(opt(newline), seeds), preceded(opt(newline), maps))),
        |(seeds, translation_maps)| State {
            seeds,
            translation_maps,
        },
    )(input)
}

fn translate(map: &BTM, index: u64) -> u64 {
    let cursor = map.upper_bound(Bound::Excluded(&index));
    match cursor.value() {
        Some(offset) => (index as i64 + offset) as u64,
        None => index,
    }
}

#[derive(Debug, PartialEq)]
struct State {
    seeds: Vec<u64>,
    translation_maps: Vec<BTM>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_numbers() {
        assert_eq!(parse_numbers("1 2 3 4 5"), Ok(("", vec![1, 2, 3, 4, 5])));
        assert_eq!(
            parse_numbers("1 2 3 4 5\n"),
            Ok(("\n", vec![1, 2, 3, 4, 5]))
        );
        assert_eq!(
            parse_numbers("1 2 3 4 5\n6 7 8 9 10"),
            Ok(("\n6 7 8 9 10", vec![1, 2, 3, 4, 5]))
        );
    }

    #[test]
    fn test_parse_seeds() {
        assert_eq!(seeds("seeds: 1 2 3 4 5"), Ok(("", (vec![1, 2, 3, 4, 5]))));
        assert_eq!(
            seeds("seeds: 1 2 3 4 5\n"),
            Ok(("\n", (vec![1, 2, 3, 4, 5])))
        );
        assert_eq!(
            seeds("seeds: 1 2 3 4 5\n6 7 8 9 10"),
            Ok(("\n6 7 8 9 10", (vec![1, 2, 3, 4, 5])))
        );
    }

    #[test]
    fn test_category() {
        assert_eq!(
            category("1 2 3").unwrap().1,
            Category {
                source: 2,
                range_length: 3,
                offset: -1
            }
        );
    }

    #[test]
    fn test_translation() {
        let seeds = vec![79u64, 14, 55, 13];
        let map: BTM = BTM::from_iter(vec![(50, 2), (50 + 48 - 1, 0), (98, -48), (98 + 2 - 1, 0)]);
        let actual: Vec<u64> = seeds.iter().map(|seed| translate(&map, *seed)).collect();
        let expected = vec![81, 14, 57, 13];
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_category_map() {
        let input = "map:
50 98 2
52 50 48";
        let expected: BTM =
            BTreeMap::from_iter(vec![(50, 2), (50 + 48 - 1, 0), (98, -48), (98 + 2 - 1, 0)]);
        assert_eq!(category_map(input).unwrap().1, expected);
    }

    #[test]
    fn test_names() {
        assert_eq!(
            map_names("seed-to-soil map:"),
            Ok((
                "map:",
                MapNames {
                    from: "seed".to_string(),
                    to: "soil".to_string()
                }
            ))
        );
    }

    #[test]
    fn test_maps() {
        let input = "seed-to-soil map:
50 98 2
52 50 48

soil-to-fertilizer map:
0 15 37
37 52 2
39 0 15

";
        let expected = vec![
            BTreeMap::from_iter(vec![(50, 2), (50 + 48 - 1, 0), (98, -48), (98 + 2 - 1, 0)]),
            BTreeMap::from_iter(vec![
                (15, -15),
                (15 + 37 - 1, 0),
                (52, -15),
                (52 + 2 - 1, 0),
                (0, 39),
                (0 + 15 - 1, 0),
            ]),
        ];
        assert_eq!(maps(input).unwrap().1, expected);
    }
    #[test]
    fn test_state() {
        let input = "seeds: 79 14 55 13

seed-to-soil map:
50 98 2
52 50 48

soil-to-fertilizer map:
0 15 37
37 52 2
39 0 15

";
        let expected = State {
            seeds: vec![79, 14, 55, 13],
            translation_maps: vec![
                BTreeMap::from_iter(vec![(50, 2), (50 + 48 - 1, 0), (98, -48), (98 + 2 - 1, 0)]),
                BTreeMap::from_iter(vec![
                    (15, -15),
                    (15 + 37 - 1, 0),
                    (52, -15),
                    (52 + 2 - 1, 0),
                    (0, 39),
                    (0 + 15 - 1, 0),
                ]),
            ],
        };
        assert_eq!(parse_state(input).unwrap().1, expected);
    }

    #[test]
    fn test_part1() {
        let input = "seeds: 79 14 55 13

seed-to-soil map:
50 98 2
52 50 48

soil-to-fertilizer map:
0 15 37
37 52 2
39 0 15

fertilizer-to-water map:
49 53 8
0 11 42
42 0 7
57 7 4

water-to-light map:
88 18 7
18 25 70

light-to-temperature map:
45 77 23
81 45 19
68 64 13

temperature-to-humidity map:
0 69 1
1 0 69

humidity-to-location map:
60 56 37
56 93 4";
        assert_eq!(run(input).unwrap().part1, "35");
    }
    #[test]
    fn test_part2() {
        let input = "seeds: 79 14 55 13

seed-to-soil map:
50 98 2
52 50 48

soil-to-fertilizer map:
0 15 37
37 52 2
39 0 15

fertilizer-to-water map:
49 53 8
0 11 42
42 0 7
57 7 4

water-to-light map:
88 18 7
18 25 70

light-to-temperature map:
45 77 23
81 45 19
68 64 13

temperature-to-humidity map:
0 69 1
1 0 69

humidity-to-location map:
60 56 37
56 93 4";
        assert_eq!(run(input).unwrap().part2, "46");
    }
}
