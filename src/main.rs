use std::fs;
use std::path::Path;
use std::str::from_utf8;

use clap::Parser;

use crate::cli::Args;
use crate::solution::Day;
use crate::DayResult::{FullSolution, SinglePart};

mod cli;
mod day1;
mod day2;
mod day3;
mod day4;
mod day5;
mod solution;

enum DayResult {
    SinglePart(String),
    FullSolution(solution::Solution),
}

fn main() {
    let args = Args::parse();

    let infile_name = format!("{:?}.txt", &args.solution).to_lowercase();
    let infile = Path::new("./inputs/").join(infile_name);
    println!("Reading input from: {:?}", &infile);
    let bytes = fs::read(infile).expect("Unable to read file");
    let input_content = from_utf8(&bytes).expect("Unable to parse file");

    println!("Running solution: {:?}", &args.solution);
    let output = match args.solution {
        Day::D1P1 => SinglePart(day1::part1::run(input_content).unwrap()),
        Day::D1P2 => SinglePart(day1::part2::run(input_content).unwrap()),
        Day::D2P1 => SinglePart(day2::part1::run(input_content).unwrap()),
        Day::D2P2 => SinglePart(day2::part2::run(input_content).unwrap()),
        Day::D3P1 => SinglePart(day3::part1::run(input_content).unwrap()),
        Day::D3P2 => SinglePart(day3::part2::run(input_content).unwrap()),
        Day::D4 => FullSolution(day4::run(input_content).unwrap()),
        Day::D5 => FullSolution(day5::run(input_content).unwrap()),
    };
    match output {
        SinglePart(output) => {
            let outfile_name = format!("{:?}.txt", &args.solution).to_lowercase();
            let outfile = Path::new("./outputs/").join(outfile_name);
            fs::write(&outfile, output).expect("Unable to write file");
            println!("Output written to: {:?}", &outfile)
        }
        FullSolution(solution) => {
            println!("Part 1: {}", solution.part1);
            println!("Part 2: {}", solution.part2);
            let p1_outfile_name = format!("{:?}p1.txt", &args.solution).to_lowercase();
            let p1_outfile = Path::new("./outputs/").join(p1_outfile_name);
            fs::write(&p1_outfile, solution.part1).expect("Unable to write file");
            println!("Part 1 output written to: {:?}", &p1_outfile);
            let p2_outfile_name = format!("{:?}p2.txt", &args.solution).to_lowercase();
            let p2_outfile = Path::new("./outputs/").join(p2_outfile_name);
            fs::write(&p2_outfile, solution.part2).expect("Unable to write file");
            println!("Part 2 output written to: {:?}", &p2_outfile);
        }
    }
}
