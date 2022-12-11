use std::collections::HashSet;

use itertools::Itertools;

use crate::Problem;

fn score(c: char) -> i64 {
    (if c.is_lowercase() {
        (c as u8 - b'a') + 1
    } else {
        (c as u8 - b'A') + 27
    }) as i64
}

pub struct Problem3;
impl Problem for Problem3 {
    fn solve_part1(&mut self, lines: &[String]) -> String {
        lines
            .iter()
            .map(|l| l.split_at(l.len() / 2))
            .map(|(one, two)| {
                *one.chars()
                    .collect::<HashSet<_>>()
                    .intersection(&two.chars().collect::<HashSet<_>>())
                    .into_iter()
                    .next()
                    .unwrap()
            })
            .map(score)
            .sum::<i64>()
            .to_string()
    }

    fn solve_part2(&mut self, lines: &[String]) -> String {
        lines
            .iter()
            .chunks(3)
            .into_iter()
            .map(|group| {
                group
                    .map(|g| g.chars().collect::<HashSet<_>>())
                    .reduce(|acc, n| acc.intersection(&n).cloned().collect::<HashSet<_>>())
                    .unwrap()
                    .into_iter()
                    .next()
                    .unwrap()
            })
            .map(score)
            .sum::<i64>()
            .to_string()
    }
}
