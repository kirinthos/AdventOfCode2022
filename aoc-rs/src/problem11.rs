use std::collections::HashSet;

use crate::Problem;

fn read_input(lines: &[String], worry_relief: bool) -> Vec<Monkey> {
    lines
        .chunks(7)
        .map(|ls| {
            let mut iter = ls.iter().skip(1);
            // just going to lazy parse this
            let items: Vec<i64> = iter
                .next()
                .unwrap()
                .trim_start_matches("  Starting items: ")
                .split(", ")
                .map(|s| s.parse().unwrap())
                .collect();
            let operation = iter
                .next()
                .unwrap()
                .trim_start_matches("  Operation: new = old ")
                .split_once(' ')
                .map(|(op, v)| match op {
                    "+" => Operation::Add(v.parse().unwrap()),
                    "*" if v != "old" => Operation::Multiply(v.parse().unwrap()),
                    "*" => Operation::Square,
                    _ => panic!("invalid operation"),
                })
                .unwrap();
            let test: i64 = iter
                .next()
                .unwrap()
                .trim_start_matches("  Test: divisible by ")
                .parse()
                .unwrap();
            let true_target: usize = iter
                .next()
                .unwrap()
                .trim_start_matches("    If true: throw to monkey ")
                .parse()
                .unwrap();
            let false_target: usize = iter
                .next()
                .unwrap()
                .trim_start_matches("    If false: throw to monkey ")
                .parse()
                .unwrap();

            Monkey {
                items,
                operation,
                test,
                true_target,
                false_target,
                inspections: 0,
                worry_relief,
            }
        })
        .collect()
}

#[derive(Debug)]
enum Operation {
    Add(i64),
    Multiply(i64),
    Square,
}

impl Operation {
    fn apply(&self, v: i64) -> i64 {
        match self {
            Operation::Add(n) => v + n,
            Operation::Multiply(n) => v * n,
            Operation::Square => v * v,
        }
    }
}

#[derive(Debug)]
struct Monkey {
    items: Vec<i64>,
    operation: Operation,
    test: i64,
    true_target: usize,
    false_target: usize,
    inspections: usize,
    worry_relief: bool,
}

impl Monkey {
    fn turn(&mut self) -> Vec<(usize, i64)> {
        let throw = self
            .items
            .iter()
            .map(|item| {
                self.inspections += 1;
                let mut item = self.operation.apply(*item);
                if self.worry_relief {
                    item /= 3;
                }

                match item % self.test == 0 {
                    true => (self.true_target, item),
                    false => (self.false_target, item),
                }
            })
            .collect();
        self.items.clear();
        throw
    }
}

pub struct Problem11;
impl Problem for Problem11 {
    fn solve_part1(&mut self, lines: &[String]) -> String {
        let mut monkeys = read_input(lines, true);
        let l = monkeys.len();

        for _ in 0..20 {
            for i in 0..l {
                for (target, item) in monkeys[i].turn() {
                    monkeys[target].items.push(item);
                }
            }
        }

        monkeys.sort_by_key(|m| m.inspections);
        monkeys
            .into_iter()
            .rev()
            .map(|m| m.inspections)
            .take(2)
            .product::<usize>()
            .to_string()
    }

    fn solve_part2(&mut self, lines: &[String]) -> String {
        let mut monkeys = read_input(lines, false);
        let ring: i64 = {
            let tests: HashSet<_> = monkeys.iter().map(|m| m.test).collect();
            tests.into_iter().product()
        };
        let l = monkeys.len();

        for _ in 0..10000 {
            for i in 0..l {
                for (target, item) in monkeys[i].turn() {
                    monkeys[target].items.push(item % ring);
                }
            }
        }

        monkeys.sort_by_key(|m| m.inspections);
        monkeys
            .into_iter()
            .rev()
            .map(|m| m.inspections)
            .take(2)
            .product::<usize>()
            .to_string()
    }
}
