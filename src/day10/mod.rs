use std::collections::{HashMap, HashSet, VecDeque};

use anyhow::anyhow;
use nom::branch::alt;
use nom::character::complete::{char, newline};
use nom::combinator::{map, opt};
use nom::multi::{many1, separated_list1};
use nom::sequence::delimited;
use nom::IResult;

use crate::solution::Solution;

#[derive(Debug, PartialEq, Copy, Clone)]
enum Pipe {
    NorthSouth,
    EastWest,
    NorthEast,
    NorthWest,
    SouthEast,
    SouthWest,
    Start,
    Ground,
}

impl Pipe {
    fn tb(&self, travel_direction: &TravelDirection) -> Option<[Vec<Coord>; 2]> {
        match (self, travel_direction) {
            (Pipe::EastWest, TravelDirection::East) => Some([
                vec![(-1, -1), (0, -1), (1, -1)],
                vec![(-1, 1), (0, 1), (1, 1)],
            ]),
            (Pipe::EastWest, TravelDirection::West) => Some([
                vec![(-1, 1), (0, 1), (1, 1)],
                vec![(-1, -1), (0, -1), (1, -1)],
            ]),
            _ => None,
        }
    }

    fn lr(&self, travel_direction: &TravelDirection) -> Option<[Vec<Coord>; 2]> {
        match (self, travel_direction) {
            (Pipe::NorthSouth, TravelDirection::North) => Some([
                vec![(-1, -1), (-1, 0), (-1, 1)],
                vec![(1, -1), (1, 0), (1, 1)],
            ]),
            (Pipe::NorthSouth, TravelDirection::South) => Some([
                vec![(1, -1), (1, 0), (1, 1)],
                vec![(-1, -1), (-1, 0), (-1, 1)],
            ]),
            _ => None,
        }
    }

    fn tl_br(&self, travel_direction: &TravelDirection) -> Option<[Vec<Coord>; 2]> {
        match (self, travel_direction) {
            (Pipe::NorthWest, TravelDirection::East) => Some([
                vec![(-1, -1)],
                vec![(-1, 1), (-1, 0), (1, -1), (1, 0), (1, 1)],
            ]),
            (Pipe::NorthWest, TravelDirection::South) => Some([
                vec![(-1, 1), (-1, 0), (1, -1), (1, 0), (1, 1)],
                vec![(-1, -1)],
            ]),
            (Pipe::SouthEast, TravelDirection::North) => Some([
                vec![(-1, -1), (-1, 0), (-1, 1), (0, -1), (1, -1)],
                vec![(1, 1)],
            ]),
            (Pipe::SouthEast, TravelDirection::East) => Some([
                vec![(1, 1)],
                vec![(-1, -1), (-1, 0), (-1, 1), (0, -1), (1, -1)],
            ]),
            _ => None,
        }
    }
    fn tr_bl(&self, travel_direction: &TravelDirection) -> Option<[Vec<Coord>; 2]> {
        match (self, travel_direction) {
            (Pipe::NorthEast, TravelDirection::South) => Some([
                vec![(1, -1)],
                vec![(-1, -1), (-1, 0), (-1, 1), (0, 1), (1, 1)],
            ]),
            (Pipe::NorthEast, TravelDirection::West) => Some([
                vec![(-1, -1), (-1, 0), (-1, 1), (0, 1), (1, 1)],
                vec![(1, -1)],
            ]),
            (Pipe::SouthWest, TravelDirection::North) => Some([
                vec![(-1, 1)],
                vec![(-1, -1), (0, -1), (1, -1), (1, 0), (1, 1)],
            ]),
            (Pipe::SouthWest, TravelDirection::East) => Some([
                vec![(-1, -1), (0, -1), (1, -1), (1, 0), (1, 1)],
                vec![(-1, 1)],
            ]),
            _ => None,
        }
    }

    fn nodes_to_check(
        &self,
        loop_direction: &LoopDirection,
        travel_direction: &TravelDirection,
    ) -> Vec<Coord> {
        let index = match loop_direction {
            LoopDirection::Clockwise => 1,
            LoopDirection::CounterClockwise => 0,
        };
        vec![
            self.tb(travel_direction),
            self.lr(travel_direction),
            self.tl_br(travel_direction),
            self.tr_bl(travel_direction),
        ]
        .iter()
        .filter_map(|x| match x {
            Some(x) => Some(x[index].iter()),
            None => None,
        })
        .flat_map(|x| x.cloned())
        .collect()
    }
}

#[derive(Debug)]
enum TravelDirection {
    North,
    South,
    East,
    West,
}

impl TravelDirection {
    fn new(x: isize, y: isize) -> anyhow::Result<Self> {
        match (x, y) {
            (0, -1) => Ok(TravelDirection::North),
            (0, 1) => Ok(TravelDirection::South),
            (1, 0) => Ok(TravelDirection::East),
            (-1, 0) => Ok(TravelDirection::West),
            _ => Err(anyhow!("Unable to infer direction")),
        }
    }
}

#[derive(Debug, PartialEq, Copy, Clone)]
enum LoopDirection {
    Clockwise,
    CounterClockwise,
}

type Coord = (isize, isize);
type Graph = HashMap<Coord, Pipe>;

fn infer_start_pipe(graph: &Graph, coord: &Coord) -> anyhow::Result<Pipe> {
    let (x, y) = coord;
    let north_coords = (x + 0, y - 1);
    let south_coords = (x + 0, y + 1);
    let east_coords = (x + 1, y + 0);
    let west_coords = (x - 1, y + 0);
    let north = graph.get(&north_coords);
    let south = graph.get(&south_coords);
    let east = graph.get(&east_coords);
    let west = graph.get(&west_coords);
    let north_connected = match north {
        Some(Pipe::NorthSouth) => true,
        Some(Pipe::SouthEast) => true,
        Some(Pipe::SouthWest) => true,
        _ => false,
    };
    let south_connected = match south {
        Some(Pipe::NorthSouth) => true,
        Some(Pipe::NorthEast) => true,
        Some(Pipe::NorthWest) => true,
        _ => false,
    };
    let east_connected = match east {
        Some(Pipe::EastWest) => true,
        Some(Pipe::NorthWest) => true,
        Some(Pipe::SouthWest) => true,
        _ => false,
    };
    let west_connected = match west {
        Some(Pipe::EastWest) => true,
        Some(Pipe::NorthEast) => true,
        Some(Pipe::SouthEast) => true,
        _ => false,
    };
    match (
        north_connected,
        south_connected,
        east_connected,
        west_connected,
    ) {
        (true, true, false, false) => Ok(Pipe::NorthSouth),
        (false, false, true, true) => Ok(Pipe::EastWest),
        (true, false, true, false) => Ok(Pipe::NorthEast),
        (true, false, false, true) => Ok(Pipe::NorthWest),
        (false, true, true, false) => Ok(Pipe::SouthEast),
        (false, true, false, true) => Ok(Pipe::SouthWest),
        _ => Err(anyhow!("Unable to infer start pipe")),
    }
}

fn next_coordinates(coord: &Coord, pipe: Option<&Pipe>) -> Option<(Coord, Coord)> {
    let (x, y) = coord;
    match pipe {
        Some(Pipe::NorthSouth) => Some(((x + 0, y + 1), (x + 0, y - 1))),
        Some(Pipe::EastWest) => Some(((x + 1, y + 0), (x - 1, y + 0))),
        Some(Pipe::NorthEast) => Some(((x + 0, y + -1), (x + 1, y + 0))),
        Some(Pipe::NorthWest) => Some(((x + 0, y + -1), (x - 1, y + 0))),
        Some(Pipe::SouthEast) => Some(((x + 0, y + 1), (x + 1, y + 0))),
        Some(Pipe::SouthWest) => Some(((x + 0, y + 1), (x - 1, y + 0))),
        _ => None,
    }
}

fn step(graph: &Graph, prev: &Coord, current: &Coord) -> Option<Coord> {
    let next = next_coordinates(&current, graph.get(&current));
    match next {
        Some((a, b)) => {
            if a == *prev {
                Some(b)
            } else {
                Some(a)
            }
        }
        None => None,
    }
}

pub fn run(input: &str) -> anyhow::Result<Solution> {
    let mut graph = parse_graph(input).expect("failed to parse").1;
    let start = find_start(&graph)
        .ok_or(anyhow!("Unable to find start"))?
        .clone();
    let start_pipe = infer_start_pipe(&graph, &start)?;
    graph.insert(start, start_pipe);
    let directions = next_coordinates(&start, graph.get(&start)).unwrap();
    let part1_solution = part1(&graph, start, &directions)?;
    let part2_solution = part2(&graph, start, &directions)?;
    Ok(Solution {
        part1: part1_solution.to_string(),
        part2: part2_solution.to_string(),
    })
}

fn walk<F>(graph: &Graph, start: &Coord, first: &Coord, mut f: F)
where
    F: FnMut(&Graph, &Coord, &Coord) -> anyhow::Result<()>,
{
    let mut prev = start.clone();
    let mut current = first.clone();
    loop {
        f(&graph, &prev, &current).unwrap();
        let next = step(&graph, &prev, &current);
        match next {
            Some(next) => {
                if current == *start {
                    break;
                }
                prev = current;
                current = next;
            }
            None => break,
        }
    }
}

fn part1(graph: &Graph, start: Coord, directions: &(Coord, Coord)) -> anyhow::Result<usize> {
    let mut steps = 0;
    let _ = walk(graph, &start, &directions.0, |_, _, _| Ok(steps += 1));
    Ok(steps / 2)
}

fn part2(graph: &Graph, start: Coord, directions: &(Coord, Coord)) -> anyhow::Result<usize> {
    let mut pipe_loop: HashSet<Coord> = HashSet::new();
    let _ = walk(graph, &start, &directions.0, |_, _, current| {
        pipe_loop.insert(current.clone());
        Ok(())
    });
    let pipe_loop = pipe_loop;
    let outside: Coord = (0, 0);
    assert_eq!(pipe_loop.contains(&outside), false);
    let outside_nodes = flood_fill(graph, &pipe_loop, &vec![&outside])?;

    let mut inside_candidates: HashSet<Coord> = HashSet::new();
    let mut loop_direction: Option<LoopDirection> = None;
    let _ = walk(graph, &start, &directions.0, |_, prev, current| {
        if let Some(_) = loop_direction {
            return Ok(());
        }
        let pipe = graph.get(&current).ok_or(anyhow!("Unable to find pipe"))?;
        let direction_of_travel = TravelDirection::new(current.0 - prev.0, current.1 - prev.1)?;
        loop_direction = match (&direction_of_travel, &pipe) {
            (TravelDirection::North, Pipe::SouthEast) => Some(LoopDirection::Clockwise),
            (TravelDirection::North, Pipe::SouthWest) => Some(LoopDirection::CounterClockwise),
            (TravelDirection::South, Pipe::NorthEast) => Some(LoopDirection::CounterClockwise),
            (TravelDirection::South, Pipe::NorthWest) => Some(LoopDirection::Clockwise),
            (TravelDirection::East, Pipe::NorthWest) => Some(LoopDirection::CounterClockwise),
            (TravelDirection::East, Pipe::SouthWest) => Some(LoopDirection::Clockwise),
            (TravelDirection::West, Pipe::NorthEast) => Some(LoopDirection::Clockwise),
            (TravelDirection::West, Pipe::SouthEast) => Some(LoopDirection::CounterClockwise),
            _ => loop_direction,
        };
        Ok(())
    });
    let loop_direction = loop_direction.ok_or(anyhow!("Unable to infer loop direction"))?;
    dbg!(loop_direction);
    let _ = walk(graph, &start, &directions.0, |_, prev, current| {
        let pipe = graph.get(&current).ok_or(anyhow!("Unable to find pipe"))?;
        let direction_of_travel = TravelDirection::new(current.0 - prev.0, current.1 - prev.1)?;
        let candidates: Vec<_> = pipe
            .nodes_to_check(&loop_direction, &direction_of_travel)
            .iter()
            .map(|x| (x.0 + current.0, x.1 + current.1))
            .collect();

        dbg!(&current, &pipe, &direction_of_travel);
        inside_candidates.extend(dbg!(candidates));
        Ok(())
    });

    assert!(inside_candidates
        .iter()
        .all(|coord| !outside_nodes.contains(coord)));
    let inside_starting_points: Vec<_> = inside_candidates
        .iter()
        .filter(|coord| !pipe_loop.contains(coord))
        .collect();
    dbg!(&inside_starting_points);

    let inside_nodes = flood_fill(graph, &pipe_loop, &inside_starting_points)?;

    Ok(inside_nodes.iter().count())
}

fn flood_fill(
    graph: &Graph,
    pipe_loop: &HashSet<Coord>,
    start: &Vec<&Coord>,
) -> anyhow::Result<HashSet<Coord>> {
    let mut out_of_loop = HashSet::new();
    let mut queue: VecDeque<Coord> = VecDeque::from_iter(start.iter().map(|x| **x));
    loop {
        let current = queue.pop_front();
        if current.is_none() {
            break;
        }
        let current = current.unwrap();
        if graph.contains_key(&current) == false {
            continue;
        }
        if pipe_loop.contains(&current) {
            continue;
        }
        if out_of_loop.contains(&current) {
            continue;
        }
        out_of_loop.insert(current.clone());
        queue.extend(vec![
            (current.0, current.1 + 1),
            (current.0, current.1 - 1),
            (current.0 + 1, current.1),
            (current.0 - 1, current.1),
        ]);
    }

    Ok(out_of_loop)
}

fn find_start(graph: &Graph) -> Option<&Coord> {
    graph
        .iter()
        .find(|(_, pipe)| **pipe == Pipe::Start)
        .map(|(coord, _)| coord)
}

fn parse_graph(input: &str) -> IResult<&str, Graph> {
    map(
        delimited(
            opt(newline),
            separated_list1(
                newline,
                many1(alt((
                    map(char('.'), |_| Pipe::Ground),
                    map(char('|'), |_| Pipe::NorthSouth),
                    map(char('-'), |_| Pipe::EastWest),
                    map(char('F'), |_| Pipe::SouthEast),
                    map(char('7'), |_| Pipe::SouthWest),
                    map(char('L'), |_| Pipe::NorthEast),
                    map(char('J'), |_| Pipe::NorthWest),
                    map(char('S'), |_| Pipe::Start),
                ))),
            ),
            opt(newline),
        ),
        |lines| {
            lines
                .iter()
                .enumerate()
                .flat_map(|(y, line)| {
                    line.iter()
                        .enumerate()
                        .map(move |(x, pipe)| ((x as isize, y as isize).clone(), (*pipe).clone()))
                })
                .collect()
        },
    )(input)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_flood_fill() -> anyhow::Result<()> {
        let input = ".....";
        let graph = parse_graph(input)?.1;
        let flood = flood_fill(&graph, &HashSet::new(), &vec![&(0, 0)])?;
        assert_eq!(
            flood,
            HashSet::from_iter(vec![(0, 0), (1, 0), (2, 0), (3, 0), (4, 0)])
        );

        let input = "..S7.\n..LJ.\n.....";
        let graph = parse_graph(input)?.1;
        let pipe_loop = HashSet::from_iter(vec![(2, 0), (3, 0), (2, 1), (3, 1)]);
        let flood = flood_fill(&graph, &pipe_loop, &vec![&(0, 0)])?;
        assert_eq!(
            flood,
            HashSet::from_iter(vec![
                (0, 0),
                (1, 0),
                (4, 0),
                (0, 1),
                (1, 1),
                (4, 1),
                (0, 2),
                (1, 2),
                (2, 2),
                (3, 2),
                (4, 2),
            ])
        );

        Ok(())
    }

    #[test]
    fn test_infer_start() -> anyhow::Result<()> {
        let input = "S-7\n|.|\nL-J";
        let graph = parse_graph(input)?.1;
        let start = find_start(&graph).ok_or(anyhow!("Unable to find start"))?;
        let actual = infer_start_pipe(&graph, start)?;
        let expected = Pipe::SouthEast;
        assert_eq!(actual, expected);
        Ok(())
    }
    #[test]
    fn test_parse_graph() -> anyhow::Result<()> {
        let input = ".....";
        let actual = parse_graph(input)?.1;
        let expected = Graph::from_iter(vec![
            ((0, 0), Pipe::Ground),
            ((1, 0), Pipe::Ground),
            ((2, 0), Pipe::Ground),
            ((3, 0), Pipe::Ground),
            ((4, 0), Pipe::Ground),
        ]);
        assert_eq!(actual, expected);

        let input = "F-7\n|.|\nL-J\nS";
        let actual = parse_graph(input)?.1;
        let expected = Graph::from_iter(vec![
            ((0, 0), Pipe::SouthEast),
            ((1, 0), Pipe::EastWest),
            ((2, 0), Pipe::SouthWest),
            ((0, 1), Pipe::NorthSouth),
            ((1, 1), Pipe::Ground),
            ((2, 1), Pipe::NorthSouth),
            ((0, 2), Pipe::NorthEast),
            ((1, 2), Pipe::EastWest),
            ((2, 2), Pipe::NorthWest),
            ((0, 3), Pipe::Start),
        ]);
        assert_eq!(actual, expected);
        Ok(())
    }

    #[test]
    fn test_part1() {
        let input = ".....
.S-7.
.|.|.
.L-J.
.....";
        assert_eq!(run(input).unwrap().part1, "4");

        let input = "-L|F7
7S-7|
L|7||
-L-J|
L|-JF";
        assert_eq!(run(input).unwrap().part1, "4");

        let input = "..F7.
.FJ|.
SJ.L7
|F--J
LJ...";
        assert_eq!(run(input).unwrap().part1, "8");

        let input = "7-F7-
.FJ|7
SJLL7
|F--J
LJ.LJ";
        assert_eq!(run(input).unwrap().part1, "8");
    }
    #[test]
    fn test_part2() {
        let input = "
..........
.S------7.
.|F----7|.
.||....||.
.||....||.
.|L-7F-J|.
.|..||..|.
.L--JL--J.
..........";
        assert_eq!(run(input).unwrap().part2, "4");
    }
}
