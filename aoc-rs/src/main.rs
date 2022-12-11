use clap::Parser;

use aoc::*;

/// Simple program to greet a person
#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    #[clap(short, long, value_parser)]
    problem: u8,
}

fn main() {
    let args = Args::parse();
    let problem_number = Problems::try_from(args.problem).unwrap();

    let example_filename = format!("../input/problem.{}.example", args.problem);
    let input_filename = format!("../input/problem.{}.input", args.problem);

    let example_lines = lines_from_file(example_filename);
    let input_lines = lines_from_file(input_filename);

    let mut problem: Box<dyn Problem> = match problem_number {
        Problems::Invalid => panic!("Invalid problem number"),
        Problems::Problem1 => Box::new(problem1::Problem1 {}),
        Problems::Problem2 => Box::new(problem2::Problem2 {}),
        Problems::Problem3 => Box::new(problem3::Problem3 {}),
        Problems::Problem4 => Box::new(problem4::Problem4 {}),
        Problems::Problem5 => Box::new(problem5::Problem5 {}),
        Problems::Problem6 => Box::new(problem6::Problem6 {}),
        Problems::Problem7 => Box::new(problem7::Problem7 {}),
        Problems::Problem8 => Box::new(problem8::Problem8 {}),
    };

    println!("part 1");
    println!("example: {}", problem.solve_part1(&example_lines));
    println!("problem: {}", problem.solve_part1(&input_lines));

    println!("\npart 2");
    println!("example: {}", problem.solve_part2(&example_lines));
    println!("problem: {}", problem.solve_part2(&input_lines));
}
