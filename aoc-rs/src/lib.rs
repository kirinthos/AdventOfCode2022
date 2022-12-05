pub mod problem1;

use std::{
    fs::File,
    io::{prelude::*, BufReader},
    path::Path,
};

#[derive(Copy, Clone)]
#[repr(u8)]
pub enum Problems {
    Invalid = 0,
    Problem1 = 1,
}

impl From<u8> for Problems {
    fn from(value: u8) -> Problems {
        match value {
            1 => Problems::Problem1,
            _ => Problems::Invalid,
        }
    }
}

pub trait Problem {
    type Return;

    fn solve_part1(&mut self, lines: &[String]) -> Self::Return;

    fn solve_part2(&mut self, lines: &[String]) -> Self::Return;
}

// helper functions
pub fn lines_from_file(filename: impl AsRef<Path>) -> Vec<String> {
    let file = File::open(filename).expect("no such file");
    let buf = BufReader::new(file);
    buf.lines()
        .map(|l| l.expect("Could not parse line"))
        .collect()
}
