use std::fmt::Debug;
use std::{fmt, fs};
use std::path::Path;

use clap::Parser;

use crate::cli::Args;
use crate::solution::Solution;

mod day1;
mod cli;
mod solution;

fn main() {
    let args = Args::parse();

    let infile_name = format!("{:?}.txt", &args.solution).to_lowercase();
    let infile = Path::new("./inputs/").join(&infile_name);
    println!("Reading input from: {:?}", &infile);
    let input_content = fs::read(infile).expect("Unable to read file");

    println!("Running solution: {:?}", &args.solution);
    let output = match args.solution {
        Solution::D1P1 => day1::part1::run(input_content),
    }.expect("Failed to run solution");

    let outfile_name = format!("{:?}.txt", &args.solution).to_lowercase();
    let outfile = Path::new("./outputs/").join(&outfile_name);
    fs::write(outfile, output).expect("Unable to write file");
}
