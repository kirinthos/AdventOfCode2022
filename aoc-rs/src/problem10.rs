use std::ops::{Deref, DerefMut};

use crate::Problem;

enum Instructions {
    Noop,
    AddX(i64),
}

struct Processor {
    instructions: Vec<Instructions>,
    ip: usize,
    // wait N cycles for current instruction pointer increase
    lag_cycles: usize,
    cycle_count: usize,
    rax: i64,
}

impl Processor {
    fn new(instructions: Vec<Instructions>) -> Self {
        let lag_cycles = match instructions[0] {
            Instructions::Noop => 0,
            Instructions::AddX(_) => 1,
        };
        Self {
            instructions,
            ip: 0,
            lag_cycles,
            cycle_count: 0,
            rax: 1,
        }
    }

    fn cycle(&mut self) {
        match self.lag_cycles {
            0 => {
                match self.instructions[self.ip] {
                    Instructions::Noop => {}
                    Instructions::AddX(addx) => self.rax += addx,
                };

                self.ip += 1;

                if self.ip < self.instructions.len() {
                    self.lag_cycles = match self.instructions[self.ip] {
                        Instructions::Noop => 0,
                        Instructions::AddX(_) => 1,
                    }
                }
            }
            _ => {
                self.lag_cycles -= 1;
            }
        };
        self.cycle_count += 1;
    }

    fn rax(&self) -> i64 {
        self.rax
    }

    fn cycle_count(&self) -> usize {
        self.cycle_count
    }
}

struct Monitor {
    monitor: [[bool; 40]; 6],
}

impl Monitor {
    fn new() -> Self {
        Monitor {
            monitor: [[false; 40]; 6],
        }
    }

    fn cycle(&mut self, processor: &Processor) {
        let c = processor.cycle_count() % 240;
        let pos = processor.rax();
        self.monitor[(c / 40) as usize][c % 40] = ((c % 40) as i64 - pos).abs() <= 1;
    }

    fn draw(&self) {
        for r in self.monitor.iter() {
            r.iter().for_each(|c| {
                print!(
                    "{}",
                    match c {
                        true => "#",
                        false => ".",
                    }
                )
            });
            println!();
        }
    }
}

struct Computer {
    processor: Processor,
    monitor: Monitor,
}

impl Computer {
    fn new(instructions: Vec<Instructions>) -> Self {
        Computer {
            processor: Processor::new(instructions),
            monitor: Monitor::new(),
        }
    }

    fn cycle(&mut self) {
        self.monitor.cycle(&self.processor);
        self.processor.cycle();
    }

    fn iter_mut(&mut self) -> Clock<'_> {
        Clock { computer: self }
    }
}

struct Clock<'a> {
    computer: &'a mut Computer,
}

impl<'a> Iterator for Clock<'a> {
    type Item = (usize, i64);

    fn next(&mut self) -> Option<Self::Item> {
        if self.computer.processor.ip >= self.computer.processor.instructions.len() {
            return None;
        }

        let v = self.computer.processor.rax();
        self.computer.cycle();
        Some((self.computer.processor.cycle_count(), v))
    }
}

fn read_input(lines: &[String]) -> Vec<Instructions> {
    lines
        .iter()
        .map(|l| {
            let mut i = l.split(' ');
            match i.next().unwrap() {
                "addx" => Instructions::AddX(i.next().unwrap().parse().unwrap()),
                "noop" => Instructions::Noop,
                _ => panic!("invalid instruction"),
            }
        })
        .collect()
}

pub struct Problem10;
impl Problem for Problem10 {
    fn solve_part1(&mut self, lines: &[String]) -> String {
        let mut computer = Computer::new(read_input(lines));
        computer
            .iter_mut()
            .map(|p| {
                println!("{:?}", p);
                p
            })
            .skip(19)
            .step_by(40)
            .take(6)
            .map(|(c, v)| {
                println!("({}, {})", c, v);
                (c as i64) * v
            })
            .sum::<i64>()
            .to_string()
    }

    fn solve_part2(&mut self, lines: &[String]) -> String {
        let mut computer = Computer::new(read_input(lines));
        let _c = computer.iter_mut().count();
        computer.monitor.draw();
        println!(
            "{}, {}, {}",
            computer.processor.ip,
            computer.processor.instructions.len(),
            computer.processor.cycle_count()
        );

        "".to_string()
    }
}
