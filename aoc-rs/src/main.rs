use clap::Parser;

use aoc::problem1;
use aoc::{lines_from_file, Problem, Problems};

/// Simple program to greet a person
#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    #[clap(short, long, value_parser)]
    problem: u8,
}

fn main() {
    let args = Args::parse();
    let problem_number: Problems = args.problem.into();

    let example_filename = format!("../input/problem.{}.example", args.problem);
    let input_filename = format!("../input/problem.{}.input", args.problem);

    let example_lines = lines_from_file(example_filename);
    let input_lines = lines_from_file(input_filename);

    let mut problem = match problem_number {
        Problems::Invalid => panic!("Invalid problem number"),
        Problems::Problem1 => problem1::Problem1 {},
    };

    println!("part 1");
    println!("example: {}", problem.solve_part1(&example_lines));
    println!("problem: {}", problem.solve_part1(&input_lines));

    println!("\npart 2");
    println!("example: {}", problem.solve_part2(&example_lines));
    println!("problem: {}", problem.solve_part2(&input_lines));
}
