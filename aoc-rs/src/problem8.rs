use itertools::Itertools;
use take_until::TakeUntilExt;

use crate::Problem;

fn read_grid(lines: &[String]) -> Vec<Vec<u32>> {
    lines
        .iter()
        .map(|l| l.chars().map(|c| c.to_digit(10).unwrap()).collect())
        .collect()
}

fn is_decreasing<T: PartialOrd + Clone + std::fmt::Debug, I: Iterator<Item = T>>(
    mut iter: I,
) -> bool {
    let tree = iter.next().unwrap();
    iter.all(|t| tree > t)
}

fn check_tree(grid: &[Vec<u32>], x: usize, y: usize) -> bool {
    [
        // up
        is_decreasing(grid.iter().take(y + 1).map(|row| row[x]).rev()),
        // down
        is_decreasing(grid.iter().skip(y).map(|row| row[x])),
        // left
        is_decreasing(grid[y].iter().take(x + 1).rev()),
        // right
        is_decreasing(grid[y].iter().skip(x)),
    ]
    .into_iter()
    .any(|p| p)
}

fn view_distance<T: PartialOrd + Clone + std::fmt::Debug, I: Iterator<Item = T>>(
    mut iter: I,
) -> usize {
    let tree = iter.next().unwrap();
    iter.take_until(|t| *t >= tree).count()
}

fn viewing_distance(grid: &[Vec<u32>], x: usize, y: usize) -> u32 {
    [
        // up
        view_distance(grid.iter().take(y + 1).map(|row| row[x]).rev()),
        // down
        view_distance(grid.iter().skip(y).map(|row| row[x])),
        // left
        view_distance(grid[y].iter().take(x + 1).rev()),
        // right
        view_distance(grid[y].iter().skip(x)),
    ]
    .into_iter()
    .product::<usize>() as u32
}

pub struct Problem8;
impl Problem for Problem8 {
    fn solve_part1(&mut self, lines: &[String]) -> String {
        let grid = read_grid(lines);

        // add the permiter, subtract 4 because each corner gets double-counted
        let mut count = grid.len() * 2 + grid[0].len() * 2 - 4;
        for y in 1..(grid.len() - 1) {
            for x in 1..(grid[0].len() - 1) {
                if check_tree(&grid, x, y) {
                    count += 1;
                }
            }
        }
        count.to_string()
    }

    fn solve_part2(&mut self, lines: &[String]) -> String {
        let grid = read_grid(lines);

        // add the permiter, subtract 4 because each corner gets double-counted
        let mut distance = 0;
        for y in 1..(grid.len() - 1) {
            for x in 1..(grid[0].len() - 1) {
                distance = distance.max(viewing_distance(&grid, x, y));
            }
        }
        distance.to_string()
    }
}
