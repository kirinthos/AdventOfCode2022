use regex;

use crate::{point::Point, Problem};

#[derive(Debug)]
struct Board {
    sensors: Vec<Sensor>,
    beacons: Vec<Point>,
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
            println!("{}", l);
            let captures = re.captures(l).unwrap();
            println!("{:?}", captures);

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
        let v = read_input(lines);
        println!("{:?}", v);
        "".to_string()
    }

    fn solve_part2(&mut self, _lines: &[String]) -> String {
        todo!()
    }
}
