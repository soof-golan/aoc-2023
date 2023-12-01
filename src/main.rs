
use std::fs;
use std::path::Path;
use std::str::from_utf8;

use clap::Parser;

use crate::cli::Args;
use crate::solution::Solution;

mod cli;
mod day1;
mod solution;

fn main() {
    let args = Args::parse();

    let infile_name = format!("{:?}.txt", &args.solution).to_lowercase();
    let infile = Path::new("./inputs/").join(&infile_name);
    println!("Reading input from: {:?}", &infile);
    let bytes = fs::read(infile).expect("Unable to read file");
    let input_content = from_utf8(&bytes).expect("Unable to parse file");

    println!("Running solution: {:?}", &args.solution);
    let output = match args.solution {
        Solution::D1P1 => day1::part1::run(input_content),
        Solution::D1P2 => day1::part2::run(input_content),
    }
    .expect("Failed to run solution");

    let outfile_name = format!("{:?}.txt", &args.solution).to_lowercase();
    let outfile = Path::new("./outputs/").join(&outfile_name);
    fs::write(&outfile, output).expect("Unable to write file");
    println!("Output written to: {:?}", &outfile)
}
