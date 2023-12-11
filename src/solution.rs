use clap::ValueEnum;

/// AOC 2023 solutions
#[derive(ValueEnum, Debug, Clone)]
#[clap(rename_all = "kebab_case")]
pub enum Day {
    D1P1,
    D1P2,
    D2P1,
    D2P2,
    D3P1,
    D3P2,
    D4,
    D5,
    D7,
    D8,
    D9,
    D10,
    D11,
}

pub struct Solution {
    pub part1: String,
    pub part2: String,
}
