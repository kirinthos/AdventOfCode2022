pub mod problem1;
pub mod problem2;
pub mod problem3;
pub mod problem4;
pub mod problem5;
pub mod problem6;
pub mod problem7;
pub mod problem8;

use num_enum::TryFromPrimitive;
use std::{
    fs::File,
    io::{prelude::*, BufReader},
    path::Path,
};

#[derive(TryFromPrimitive, Copy, Clone)]
#[repr(u8)]
pub enum Problems {
    Invalid = 0,
    Problem1,
    Problem2,
    Problem3,
    Problem4,
    Problem5,
    Problem6,
    Problem7,
    Problem8,
}

pub trait Problem {
    fn solve_part1(&mut self, lines: &[String]) -> String;

    fn solve_part2(&mut self, lines: &[String]) -> String;
}

// helper functions
pub fn lines_from_file(filename: impl AsRef<Path>) -> Vec<String> {
    let file = File::open(filename).expect("no such file");
    let buf = BufReader::new(file);
    buf.lines()
        .map(|l| l.expect("Could not parse line"))
        .collect()
}

pub fn transpose<T: Clone>(v: Vec<Vec<T>>) -> Vec<Vec<T>> {
    (0..v[0].len())
        .map(|i| v.iter().map(|inner| inner[i].clone()).collect())
        .collect()
}
