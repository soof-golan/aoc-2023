
use clap::ValueEnum;

/// AOC 2023 solutions
#[derive(ValueEnum, Debug, Clone)]
#[clap(rename_all = "kebab_case")]
pub enum Solution {
    D1P1
}

