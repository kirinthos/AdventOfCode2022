use itertools::Itertools;

use crate::{transpose, Problem};

pub struct Problem5;
impl Problem5 {
    fn read_stacks(&self, lines: &[String]) -> Vec<Vec<char>> {
        // can't use iterator because we need `rev()`, DoubleEndedIterator
        #[allow(clippy::needless_collect)]
        let s: Vec<Vec<char>> = lines
            .iter()
            .take_while(|l| !l.is_empty())
            .map(|l| l.chars().collect())
            .collect();

        transpose(s.into_iter().rev().collect())
            .into_iter()
            // skip the left-most column
            .skip(1)
            // contents are in every 4th column, including 0 (because we skip the useless column above)
            .step_by(4)
            // skip the column numeric label and remove empty entries
            .map(|row| row.into_iter().skip(1).filter(|&c| c != ' ').collect())
            .collect()
    }

    fn read_instructions(&self, lines: &[String]) -> Vec<(usize, usize, usize)> {
        lines
            .iter()
            // skip stacks at beginning of input
            .skip_while(|l| !l.is_empty())
            // skip blank line
            .skip(1)
            .map(|l| {
                // split line into words
                l.split(' ')
                    // the first word is 'move'
                    .skip(1)
                    // take the numbers in the instruction
                    .step_by(2)
                    .map(|s| s.parse::<usize>().unwrap())
                    // turn them into a tuple of (move, from, to)
                    .collect_tuple()
                    .unwrap()
            })
            .collect()
    }
}

impl Problem for Problem5 {
    fn solve_part1(&mut self, lines: &[String]) -> String {
        let mut stacks = self.read_stacks(lines);
        let instructions = self.read_instructions(lines);

        for (m, f, t) in instructions.into_iter() {
            for _ in 0..m {
                let n = stacks[f - 1].pop().unwrap();
                stacks[t - 1].push(n);
            }
        }

        stacks.into_iter().map(|s| *s.last().unwrap()).collect()
    }

    fn solve_part2(&mut self, lines: &[String]) -> String {
        let mut stacks = self.read_stacks(lines);
        let instructions = self.read_instructions(lines);

        for (m, f, t) in instructions.into_iter() {
            let l = stacks[f - 1].len();
            let mut n = stacks[f - 1].drain((l - m)..).collect();
            stacks[t - 1].append(&mut n);
        }

        stacks.into_iter().map(|s| *s.last().unwrap()).collect()
    }
}
