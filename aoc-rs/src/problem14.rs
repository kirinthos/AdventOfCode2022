use std::collections::HashSet;

use itertools::Itertools;

use crate::{point::Point, Problem};

const SAND_X: i32 = 500;

#[derive(Debug)]
pub struct Board {
    board: HashSet<Point>,
    largest_y: i32,
    part2: bool,
    end: Point,
}

impl Board {
    fn new(board: Vec<Point>, part2: bool) -> Self {
        let mut largest_y = board.iter().map(|p| p.y()).max().unwrap();
        if part2 {
            // the floor is +2 but our check is a <, so only + 1
            largest_y += 1;
        }
        Self {
            board: HashSet::from_iter(board.into_iter()),
            largest_y,
            part2,
            end: Point::new(SAND_X, 0),
        }
    }

    fn drop_sand(&mut self) -> bool {
        let mut sand = Point::new(SAND_X, 0);
        while sand.y() < self.largest_y {
            let f = [
                sand + (0, 1).into(),
                sand + (-1, 1).into(),
                sand + (1, 1).into(),
            ]
            .into_iter()
            .find(|p| !self.board.contains(p));

            match f {
                Some(p) => sand = p,
                None => {
                    self.board.insert(sand);
                    // part2
                    if sand == self.end {
                        return false;
                    }
                    // continue game
                    return true;
                }
            }
        }

        if self.part2 {
            self.board.insert(sand);
            true
        } else {
            // end of game
            false
        }
    }
}

fn read_input(lines: &[String]) -> Vec<Point> {
    lines
        .iter()
        .flat_map(|l| {
            let iter = l.split(" -> ").map(|p| {
                let p = p.split_once(',').unwrap();
                Point::new(p.0.parse().unwrap(), p.1.parse().unwrap())
            });

            iter.clone().zip(iter.skip(1)).flat_map(|(p1, p2)| {
                if p1.x() == p2.x() {
                    (p1.y().min(p2.y())..=p1.y().max(p2.y()))
                        .map(|y| (p1.x(), y))
                        .map(|p| p.into())
                        .collect_vec()
                } else {
                    (p1.x().min(p2.x())..=p1.x().max(p2.x()))
                        .map(|x| (x, p1.y()))
                        .map(|p| p.into())
                        .collect_vec()
                }
            })
        })
        .collect()
}

pub struct Problem14 {}

impl Problem for Problem14 {
    fn solve_part1(&mut self, lines: &[String]) -> String {
        let v = read_input(lines);
        let mut board = Board::new(v, false);
        let mut i = 0;
        while board.drop_sand() {
            i += 1;
        }
        i.to_string()
    }

    fn solve_part2(&mut self, lines: &[String]) -> String {
        let v = read_input(lines);
        let mut board = Board::new(v, true);
        let mut i = 0;
        while board.drop_sand() {
            i += 1;
        }
        (i + 1).to_string()
    }
}
