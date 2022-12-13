use std::collections::HashSet;

use crate::point::Point;
use crate::Problem;

#[derive(Debug, Clone)]
struct Snake {
    body: Vec<Point>,
}

impl Snake {
    fn new(length: usize) -> Self {
        Self {
            body: std::iter::repeat(Point::new(0, 0)).take(length).collect(),
        }
    }

    fn shift(&mut self, x: i32, y: i32) {
        self.body[0].shift(x, y);
        let mut head = self.body[0];
        for tail in self.body.iter_mut().skip(1) {
            let (xdist, ydist) = (head.x() - tail.x(), head.y() - tail.y());
            if xdist.abs() == 2 || ydist.abs() == 2 {
                tail.shift(xdist.signum(), ydist.signum());
            }
            head = *tail;
        }
    }
}

pub struct Problem9;
impl Problem for Problem9 {
    fn solve_part1(&mut self, lines: &[String]) -> String {
        let moves = lines
            .iter()
            .map(|l| l.split_once(' ').unwrap())
            .flat_map(|(dir, n)| {
                let n: i32 = n.parse().unwrap();
                match dir {
                    "R" => std::iter::repeat((1, 0)).take(n as usize),
                    "L" => std::iter::repeat((-1, 0)).take(n.abs() as usize),
                    "U" => std::iter::repeat((0, 1)).take(n as usize),
                    "D" => std::iter::repeat((0, -1)).take(n.abs() as usize),
                    _ => panic!("invalid direction"),
                }
            });

        let mut snake = Snake::new(2);
        let mut tails: HashSet<Point, _> = HashSet::new();
        tails.extend(snake.body.iter().skip(1));
        for (x, y) in moves {
            snake.shift(x, y);
            tails.extend(snake.body.iter().skip(1));
        }
        tails.len().to_string()
    }

    fn solve_part2(&mut self, lines: &[String]) -> String {
        let moves = lines
            .iter()
            .map(|l| l.split_once(' ').unwrap())
            .flat_map(|(dir, n)| {
                let n: i32 = n.parse().unwrap();
                match dir {
                    "R" => std::iter::repeat((1, 0)).take(n as usize),
                    "L" => std::iter::repeat((-1, 0)).take(n.abs() as usize),
                    "U" => std::iter::repeat((0, 1)).take(n as usize),
                    "D" => std::iter::repeat((0, -1)).take(n.abs() as usize),
                    _ => panic!("invalid direction"),
                }
            });

        let mut snake = Snake::new(10);
        let mut tails: HashSet<Point, _> = HashSet::new();
        tails.insert(*snake.body.last().unwrap());
        for (x, y) in moves {
            snake.shift(x, y);
            tails.insert(*snake.body.last().unwrap());
        }
        tails.len().to_string()
    }
}
