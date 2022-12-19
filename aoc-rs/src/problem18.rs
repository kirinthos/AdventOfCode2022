use std::collections::HashSet;

use crate::{point::Point3D, Problem};

fn read_input(lines: &[String]) -> HashSet<Point3D> {
    lines
        .iter()
        .map(|l| {
            let coords = l.split(',').collect::<Vec<_>>();
            Point3D::new(
                coords[0].parse().unwrap(),
                coords[1].parse().unwrap(),
                coords[2].parse().unwrap(),
            )
        })
        .collect()
}

pub struct Problem18;
impl Problem for Problem18 {
    fn solve_part1(&mut self, lines: &[String]) -> String {
        let cubes = read_input(lines);
        let directions = [
            Point3D::new(1, 0, 0),
            Point3D::new(-1, 0, 0),
            Point3D::new(0, 1, 0),
            Point3D::new(0, -1, 0),
            Point3D::new(0, 0, 1),
            Point3D::new(0, 0, -1),
        ];
        let mut sides_exposed = 0;
        for c in cubes.iter() {
            sides_exposed += directions
                .iter()
                .filter(|d| !cubes.contains(&(*c + **d)))
                .count();
        }
        sides_exposed.to_string()
    }

    fn solve_part2(&mut self, lines: &[String]) -> String {
        let cubes = read_input(lines);
        crawl(Point3D::new(0, 0, 0), &cubes, &mut HashSet::new()).to_string()
    }
}

fn crawl(point: Point3D, cubes: &HashSet<Point3D>, visited: &mut HashSet<Point3D>) -> i32 {
    // out of bounds
    if point.x() < -1
        || point.y() < -1
        || point.z() < -1
        || point.x() > 20
        || point.y() > 20
        || point.z() > 20
    {
        return 0;
    }

    // we touched something
    if cubes.contains(&point) {
        return 1;
    }

    if visited.contains(&point) {
        return 0;
    }

    visited.insert(point);

    let directions = [
        Point3D::new(-1, 0, 0),
        Point3D::new(1, 0, 0),
        Point3D::new(0, -1, 0),
        Point3D::new(0, 1, 0),
        Point3D::new(0, 0, -1),
        Point3D::new(0, 0, 1),
    ];

    let mut sides_exposed = 0;
    for d in directions.into_iter() {
        sides_exposed += crawl(point + d, cubes, visited);
    }
    sides_exposed
}
