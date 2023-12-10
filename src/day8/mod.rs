use std::collections::{HashMap, HashSet};
use std::iter::zip;
use std::ops::ControlFlow::{Break, Continue};

use anyhow;
use nom::bytes::complete::{tag, take};
use nom::character::complete::{newline, one_of};
use nom::combinator::{map, opt};
use nom::multi::{fold_many1, many1};
use nom::sequence::{delimited, separated_pair, terminated};
use nom::IResult;
use num::Integer;

use crate::solution::Solution;

pub fn run(input: &str) -> anyhow::Result<Solution> {
    let (_, (instructions, graph)) = instructions_and_graph(input).expect("failed to parse");

    let part1_solution = match graph.contains_key("AAA") {
        false => 0usize,
        true => part1(
            &instructions,
            &graph,
            &"AAA",
            &HashSet::from_iter(vec![&"ZZZ"]),
        ),
    };

    let part2_solution = part2(&instructions, &graph);

    Ok(Solution {
        part1: part1_solution.to_string(),
        part2: part2_solution.to_string(),
    })
}

type StartingVertex = u16;
type InstructionIndex = u16;
type PathLength = usize;
type PathKey = (StartingVertex, InstructionIndex);
type PathsMap = HashMap<PathKey, PathLength>;
fn compute_paths(
    instructions: &Vec<bool>,
    graph: &Vec<[u16; 2]>,
    starting_vertices: &Vec<u16>,
    sentinels: &Vec<bool>,
) -> PathsMap {
    starting_vertices
        .iter()
        .flat_map(|start| {
            let mut visited: PathsMap = HashMap::new();
            let _ = instructions
                .iter()
                .enumerate()
                .cycle()
                .enumerate()
                .try_fold(start.clone(), |current, (length, (offset, lr))| {
                    let next = graph[current as usize][*lr as usize];
                    let key = (*start, offset as u16);
                    if !(visited.contains_key(&key)) {
                        if sentinels[current as usize] {
                            visited.insert(key, length);
                        }
                        Continue(next)
                    } else {
                        Break(current)
                    }
                });

            visited
        })
        .collect()
}

fn part2(instructions: &Vec<Instruction>, graph: &Graph) -> PathLength {
    let mut sorted_keys: Vec<Vertex> = graph.keys().map(|k| *k).collect();
    sorted_keys.sort();

    let vertex_to_int: HashMap<Vertex, u16> =
        HashMap::from_iter(sorted_keys.iter().enumerate().map(|(i, k)| (*k, i as u16)));
    // let int_to_vertex: HashMap<u16, Vertex> =
    //     HashMap::from_iter(sorted_keys.iter().enumerate().map(|(i, k)| (i as u16, *k)));
    let left: Vec<u16> = sorted_keys
        .iter()
        .map(|k| graph[k].0)
        .map(|k| vertex_to_int[k])
        .collect();
    let right: Vec<u16> = sorted_keys
        .iter()
        .map(|k| graph[k].1)
        .map(|k| vertex_to_int[k])
        .collect();
    let both: Vec<[u16; 2]> = zip(left, right).map(|(l, r)| [l, r]).collect();
    let sentinels: Vec<bool> = sorted_keys.iter().map(|k| k.ends_with("Z")).collect();
    let starting_vertices: Vec<u16> = graph
        .keys()
        .filter(|k| k.ends_with("A"))
        .map(|k| vertex_to_int[k])
        .collect::<Vec<u16>>();
    let instructions: Vec<bool> = instructions
        .iter()
        .map(|i| match i {
            Instruction::Left => false,
            Instruction::Right => true,
        })
        .collect();
    let paths = compute_paths(&instructions, &both, &starting_vertices, &sentinels);
    dbg!(&paths);
    paths.values().copied().reduce(|a, b| a.lcm(&b)).unwrap()
}

fn part1<'a>(
    instructions: &Vec<Instruction>,
    graph: &Graph<'a>,
    start: &Vertex<'a>,
    sentinels: &HashSet<&Vertex<'a>>,
) -> usize {
    let mut current: Vertex = start;
    for (i, instruction) in instructions.iter().cycle().enumerate() {
        current = match instruction {
            Instruction::Left => graph[current].0,
            Instruction::Right => graph[current].1,
        };
        if sentinels.contains(&current) {
            return i + 1;
        }
    }
    unreachable!()
}

#[derive(Debug, PartialEq)]
enum Instruction {
    Right,
    Left,
}

#[inline]
fn instruction(input: &str) -> IResult<&str, Instruction> {
    map(one_of("LR"), |c| match c {
        'L' => Instruction::Left,
        'R' => Instruction::Right,
        _ => unreachable!(),
    })(input)
}

fn instructions(input: &str) -> IResult<&str, Vec<Instruction>> {
    terminated(many1(instruction), opt(newline))(input)
}

type Vertex<'a> = &'a str;
type Graph<'a> = HashMap<Vertex<'a>, (Vertex<'a>, Vertex<'a>)>;
fn node<'a>(input: &'a str) -> IResult<&str, (Vertex<'a>, (Vertex<'a>, Vertex<'a>))> {
    separated_pair(
        take(3usize),
        tag(" = "),
        delimited(
            tag("("),
            separated_pair(take(3usize), tag(", "), take(3usize)),
            tag(")"),
        ),
    )(input)
}

fn graph<'a>(input: &'a str) -> IResult<&str, Graph<'a>> {
    fold_many1(
        terminated(node, opt(newline)),
        Graph::new,
        |mut acc, (k, v)| {
            acc.insert(k, v);
            acc
        },
    )(input)
}

fn instructions_and_graph<'a>(input: &'a str) -> IResult<&str, (Vec<Instruction>, Graph<'a>)> {
    separated_pair(instructions, newline, graph)(input)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_instruction() {
        assert_eq!(instruction("R"), Ok(("", Instruction::Right)));
        assert_eq!(instruction("L"), Ok(("", Instruction::Left)));
    }

    #[test]
    fn test_parse_instructions() {
        assert_eq!(
            instructions("LLR"),
            Ok((
                "",
                vec![Instruction::Left, Instruction::Left, Instruction::Right]
            ))
        );
        assert_eq!(
            instructions("RLRL"),
            Ok((
                "",
                vec![
                    Instruction::Right,
                    Instruction::Left,
                    Instruction::Right,
                    Instruction::Left
                ]
            ))
        );
    }

    #[test]
    fn test_parse_node() {
        let input = "AAA = (BBB, CCC)";
        assert_eq!(node(input), Ok(("", ("AAA", ("BBB", "CCC")))));
    }
    #[test]
    fn test_parse_nodes() {
        let input = "AAA = (BBB, CCC)
BBB = (DDD, EEE)
";
        let expected = Graph::from_iter(vec![("AAA", ("BBB", "CCC")), ("BBB", ("DDD", "EEE"))]);
        assert_eq!(graph(input), Ok(("", expected)));
    }

    #[test]
    fn test_instructions_and_graph() {
        let input = "RL

AAA = (BBB, CCC)
BBB = (DDD, EEE)";
        let expected = (
            vec![Instruction::Right, Instruction::Left],
            Graph::from_iter(vec![("AAA", ("BBB", "CCC")), ("BBB", ("DDD", "EEE"))]),
        );
        assert_eq!(instructions_and_graph(input), Ok(("", expected)));
    }

    ///LLR
    //
    // AAA = (BBB, BBB)
    // BBB = (AAA, ZZZ)
    // ZZZ = (ZZZ, ZZZ)
    #[test]
    fn test_paths() {
        let instructions = vec![false, false, true];
        let graph = vec![[1u16, 1], [0, 2], [2, 2]];
        let starting_vertices = vec![0u16];
        let sentinels = vec![false, false, true];
        let actual = compute_paths(&instructions, &graph, &starting_vertices, &sentinels);
        let expected: PathsMap = HashMap::from_iter(vec![((0, 0), 6), ((0, 1), 7), ((0, 2), 8)]);
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_part1() {
        let input = "RL

AAA = (BBB, CCC)
BBB = (DDD, EEE)
CCC = (ZZZ, GGG)
DDD = (DDD, DDD)
EEE = (EEE, EEE)
GGG = (GGG, GGG)
ZZZ = (ZZZ, ZZZ)";
        assert_eq!(run(input).unwrap().part1, "2");
        let input = "LLR

AAA = (BBB, BBB)
BBB = (AAA, ZZZ)
ZZZ = (ZZZ, ZZZ)";
        assert_eq!(run(input).unwrap().part1, "6");
    }
    #[test]
    fn test_part2() {
        let input = "LR

11A = (11B, XXX)
11B = (XXX, 11Z)
11Z = (11B, XXX)
22A = (22B, XXX)
22B = (22C, 22C)
22C = (22Z, 22Z)
22Z = (22B, 22B)
XXX = (XXX, XXX)";
        assert_eq!(run(input).unwrap().part2, "6");
    }
}
