use std::collections::HashSet;

use itertools::Itertools;

use crate::solution::Solution;

pub fn run(input: &str) -> anyhow::Result<Solution> {
    let part1_solution = part1(input)?;
    let part2_solution = part2(input)?;
    Ok(Solution {
        part1: part1_solution.to_string(),
        part2: part2_solution.to_string(),
    })
}
fn part1(input: &str) -> anyhow::Result<usize> {
    let universe = expand_universe(input, 2);
    Ok(min_distance_pairs(universe).iter().sum())
}

fn part2(input: &str) -> anyhow::Result<usize> {
    let universe = expand_universe(input, 1_000_000);
    Ok(min_distance_pairs(universe).iter().sum())
}

type Galaxy = (usize, usize);
type Universe = Vec<Galaxy>;

fn universe(input: &str) -> Universe {
    input
        .strip_prefix("\n")
        .unwrap_or(input)
        .strip_suffix("\n")
        .unwrap_or(input)
        .lines()
        .enumerate()
        .flat_map(move |(y, line)| {
            line.chars()
                .enumerate()
                .filter_map(move |(x, c)| if c == '#' { Some((x, y)) } else { None })
        })
        .collect()
}

fn expand_universe(input: &str, expansion_factor: usize) -> Universe {
    let offset = expansion_factor - 1;
    let max_x = input.lines().next().unwrap().len();
    let max_y = input.lines().count();
    let small_universe = universe(input);
    let rows_with_galaxies: HashSet<_> =
        small_universe.iter().map(|(_, y)| *y).into_iter().collect();
    let cols_with_galaxies: HashSet<_> =
        small_universe.iter().map(|(x, _)| *x).into_iter().collect();

    let x_offsets = (0..max_x)
        .scan(0usize, |acc, x| {
            if !cols_with_galaxies.contains(&x) {
                *acc += offset;
            }
            Some(*acc)
        })
        .collect::<Vec<_>>();

    let y_offsets = (0..max_y)
        .scan(0usize, |acc, y| {
            if !(rows_with_galaxies.contains(&y)) {
                *acc += offset;
            }
            Some(*acc)
        })
        .collect::<Vec<_>>();

    small_universe
        .iter()
        .map(|(x, y)| {
            let x_offset = x_offsets[*x];
            let y_offset = y_offsets[*y];
            (*x + x_offset, *y + y_offset)
        })
        .collect::<Universe>()
}

fn l1(ab: (&Galaxy, &Galaxy)) -> usize {
    let (a, b) = ab;
    return ((a.0 as isize - b.0 as isize).abs() + (a.1 as isize - b.1 as isize).abs()) as usize;
}

fn min_distance_pairs(universe: Universe) -> Vec<usize> {
    universe
        .iter()
        .combinations(2)
        .map(|ab| (ab[0], ab[1]))
        .map(l1)
        .sorted()
        .collect::<Vec<_>>()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_min_distance() {
        let input = "\
.#..#
.....
#....";
        let expected = vec![3, 3, 6];
        let universe = universe(input);
        let distance_pairs = min_distance_pairs(universe);
        assert_eq!(distance_pairs, expected);
    }

    #[test]
    fn test_universe() {
        let input = ".#.\n..#";
        let expected = vec![(1, 0), (2, 1)];
        assert_eq!(universe(input), expected);
    }

    #[test]
    fn test_expand_universe() -> anyhow::Result<()> {
        let input = ".#.
...
..#";
        let expected = "..#.
....
....
...#";
        assert_eq!(expand_universe(input, 2), universe(expected));
        Ok(())
    }

    #[test]
    fn test_part1() {
        let input = "...#......
.......#..
#.........
..........
......#...
.#........
.........#
..........
.......#..
#...#.....";
        assert_eq!(run(input).unwrap().part1, "374");
        assert_eq!(run(input).unwrap().part1, "374");
    }
    #[test]
    fn test_part2() -> anyhow::Result<()> {
        let input = "...#......
.......#..
#.........
..........
......#...
.#........
.........#
..........
.......#..
#...#.....";
        let universe = expand_universe(input, 10);
        assert_eq!(
            min_distance_pairs(universe).iter().sum::<usize>(),
            1030usize
        );
        let universe = expand_universe(input, 100);
        assert_eq!(
            min_distance_pairs(universe).iter().sum::<usize>(),
            8410usize
        );
        Ok(())
    }
}
