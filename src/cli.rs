use clap::Parser;
use crate::solution::Solution;


/// Simple program to greet a person
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Args {
    /// Which AOC day + part to run
    #[arg()]
    pub(crate) solution: Solution,

}