use std::collections::HashSet;

use regex;

use crate::{point::Point, Problem};

#[derive(Debug)]
struct Board {
    sensors: Vec<Sensor>,
    beacons: HashSet<Point>,
}

impl Board {
    fn row_coverage(&self, y: i32) -> HashSet<Point> {
        let covered: HashSet<_> = self
            .sensors
            .iter()
            .flat_map(|s| {
                // radius - distance to the row = amount of movement left/right
                let d = (s.location.y() - y).abs() - s.radius;
                match d.signum() {
                    // positive result means the sensor is too far away for radius to reach
                    1 => vec![],
                    // radius exactly reaches, only one point
                    0 => vec![Point::new(s.location.x(), y)],
                    // radius crosses the row, so emit points to the left/right,
                    // recall d is negative after radius subtraction
                    -1 => (d..=-d)
                        .map(|i| Point::new(s.location.x() + i, y))
                        .collect(),
                    _ => panic!("uh, signum should be [-1,1]"),
                }
            })
            .collect();
        &covered - &self.beacons
    }
}

#[derive(Debug)]
struct Sensor {
    location: Point,
    radius: i32,
}

impl Sensor {
    fn contains(&self, p: &Point) -> bool {
        self.location.manhattan_distance(p) <= self.radius
    }
}

fn read_input(lines: &[String]) -> Board {
    let re = regex::Regex::new(
        r#"Sensor at x=(-?\d+), y=(-?\d+): closest beacon is at x=(-?\d+), y=(-?\d+)"#,
    )
    .unwrap();
    let (sensors, beacons) = lines
        .iter()
        .map(|l| {
            let captures = re.captures(l).unwrap();

            let location = Point::new(
                captures.get(1).unwrap().as_str().parse().unwrap(),
                captures.get(2).unwrap().as_str().parse().unwrap(),
            );
            let beacon = Point::new(
                captures.get(3).unwrap().as_str().parse().unwrap(),
                captures.get(4).unwrap().as_str().parse().unwrap(),
            );
            let sensor = Sensor {
                location,
                radius: location.manhattan_distance(&beacon),
            };

            (sensor, beacon)
        })
        .unzip();

    Board { sensors, beacons }
}

pub struct Problem15;
impl Problem for Problem15 {
    fn solve_part1(&mut self, lines: &[String]) -> String {
        let board = read_input(lines);
        //let coverage_example = board.row_coverage(10).len();
        let coverage_input = board.row_coverage(2_000_000).len();
        coverage_input.to_string()
    }

    fn solve_part2(&mut self, lines: &[String]) -> String {
        // we only need to check the points along the perimeter of each sensor
        let board_limit = (4_000_000, 4_000_000); // (20, 20)
        let board = read_input(lines);
        let p = board
            .sensors
            .iter()
            .flat_map(|s| {
                let (xmin, xmax) = (s.location.x() - s.radius - 1, s.location.x() + s.radius + 1);

                (xmin..=xmax).flat_map(|x| {
                    let y_dist = (s.radius + 1) - (s.location.x() - x).abs();
                    [
                        Point::new(x, s.location.y() + y_dist),
                        Point::new(x, s.location.y() - y_dist),
                    ]
                })
            })
            .find(|p| {
                (p.x() >= 0 && p.y() >= 0 && p.x() <= board_limit.0 && p.y() <= board_limit.1)
                    && board.sensors.iter().all(|s| !s.contains(p))
            })
            .unwrap();
        (p.x() as i64 * 4_000_000 + p.y() as i64).to_string()
    }
}
