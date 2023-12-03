use clap::ValueEnum;

/// AOC 2023 solutions
#[derive(ValueEnum, Debug, Clone)]
#[clap(rename_all = "kebab_case")]
pub enum Solution {
    D1P1,
    D1P2,
    D2P1,
    D2P2,
    D3P1,
    D3P2,
}
