use std::collections::HashSet;

use itertools::Itertools;

use crate::Problem;

pub struct Problem6;
impl Problem6 {
    fn find_packet(&self, lines: &[String], n: usize) -> String {
        lines
            .iter()
            .map(|s| s.chars().collect::<Vec<_>>())
            .map(|s| {
                // walk window of n characters
                (s.windows(n)
                    // if unique length is not n, keep walking
                    .take_while(|w| HashSet::<_>::from_iter(w.iter()).len() != n)
                    // count the number of windows we saw
                    .count()
                    + n) // add n because of the window size
                    .to_string()
            })
            .join("\n")
    }
}

impl Problem for Problem6 {
    fn solve_part1(&mut self, lines: &[String]) -> String {
        self.find_packet(lines, 4)
    }

    fn solve_part2(&mut self, lines: &[String]) -> String {
        self.find_packet(lines, 14)
    }
}
