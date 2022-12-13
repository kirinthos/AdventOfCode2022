use std::collections::VecDeque;

use crate::point::Point;
use crate::Problem;

/// Outputs the start point, the end point, and the grid of mountains
fn read_input(lines: &[String]) -> (Point, Point, Vec<Vec<i32>>) {
    let mut start = (0, 0);
    let mut end = (0, 0);
    let grid = lines
        .iter()
        .enumerate()
        .map(|(y, l)| {
            l.chars()
                .enumerate()
                .map(|(x, c)| match c {
                    'S' => {
                        start = (x, y);
                        0
                    }
                    'E' => {
                        end = (x, y);
                        25
                    }
                    _ => ((c as u8) - b'a') as i32,
                })
                .collect()
        })
        .collect();

    (start.into(), end.into(), grid)
}

fn available_moves(p: &Point, grid: &[Vec<i32>], exploration: &[Vec<Option<i32>>]) -> Vec<Point> {
    let (lx, ly) = (grid[0].len() as i32, grid.len() as i32);
    [(-1, 0), (1, 0), (0, -1), (0, 1)]
        .into_iter()
        .filter_map(|(x, y)| {
            let n = *p + (x, y).into();

            let (x, y) = (n.x(), n.y());

            if x < 0
                || y < 0
                || x >= lx
                || y >= ly
                // if the next square is more than 1 higher than the current square
                || (grid[y as usize][x as usize] - grid[p.y() as usize][p.x() as usize]) > 1
                // if the next square's score is not None and its value is <= to
                // what this path's score would be
                || exploration[y as usize][x as usize].map_or(false, |n_score| {
                    n_score <= exploration[p.y() as usize][p.x() as usize].unwrap() + 1
                })
            {
                // do nothing
                None
            } else {
                Some(n)
            }
        })
        .collect()
}

fn explore(grid: &Vec<Vec<i32>>, start: Point) -> Vec<Vec<Option<i32>>> {
    let mut exploration: Vec<Vec<Option<i32>>> =
        std::iter::repeat(std::iter::repeat(None).take(grid[0].len()))
            .take(grid.len())
            .map(|r| r.collect())
            .collect();

    let mut moves = VecDeque::new();
    exploration[start.y() as usize][start.x() as usize] = Some(0);
    moves.push_back(start);
    while !moves.is_empty() {
        let next = moves.pop_front().unwrap();
        let available = available_moves(&next, grid, &exploration);

        // we could just map it, but we want to panic! so we can see errors
        let s = exploration[next.y() as usize][next.x() as usize].unwrap();
        for m in available {
            exploration[m.y() as usize][m.x() as usize] = Some(s + 1);
            moves.push_back(m);
        }
    }

    exploration
}

pub struct Problem12;
impl Problem for Problem12 {
    fn solve_part1(&mut self, lines: &[String]) -> String {
        let (start, end, grid) = read_input(lines);
        let explored_grid = explore(&grid, start);

        explored_grid[end.y() as usize][end.x() as usize]
            .unwrap()
            .to_string()
    }

    fn solve_part2(&mut self, lines: &[String]) -> String {
        // Note: there's an optimization in here to use the same explored grid over and over
        // but the input so small that it will still run fast, regardless
        let (_, end, grid) = read_input(lines);
        let choices = grid.iter().enumerate().flat_map(|(y, row)| {
            row.iter()
                .enumerate()
                .filter_map(move |(x, height)| match *height == 0 {
                    true => Some(Point::from((x, y))),
                    false => None,
                })
        });

        choices
            .into_iter()
            .filter_map(|start| explore(&grid, start)[end.y() as usize][end.x() as usize])
            .min()
            .unwrap()
            .to_string()
    }
}
