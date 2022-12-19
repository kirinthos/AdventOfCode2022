use regex;

use crate::Problem;

#[derive(Debug, Copy, Clone)]
struct Blueprint {
    ore_robot: usize,
    clay_robot: usize,
    obsidian_robot: (usize, usize),
    geode_robot: (usize, usize),
}

fn read_input(lines: &[String]) -> Vec<Blueprint> {
    let re = regex::Regex::new(r#"costs (\d+) ore(?: and (\d+) \w+)?."#).unwrap();

    lines
        .iter()
        .map(|l| {
            let mut captures = re.captures_iter(l);

            let ore_robot = captures
                .next()
                .unwrap()
                .get(1)
                .unwrap()
                .as_str()
                .parse()
                .unwrap();
            let clay_robot = captures
                .next()
                .unwrap()
                .get(1)
                .unwrap()
                .as_str()
                .parse()
                .unwrap();

            let next = captures.next().unwrap();
            let obsidian_robot = (
                next.get(1).unwrap().as_str().parse().unwrap(),
                next.get(2).unwrap().as_str().parse().unwrap(),
            );
            let next = captures.next().unwrap();
            let geode_robot = (
                next.get(1).unwrap().as_str().parse().unwrap(),
                next.get(2).unwrap().as_str().parse().unwrap(),
            );

            Blueprint {
                ore_robot,
                clay_robot,
                obsidian_robot,
                geode_robot,
            }
        })
        .collect()
}

pub struct Problem19;
impl Problem for Problem19 {
    fn solve_part1(&mut self, lines: &[String]) -> String {
        let blueprints = read_input(lines);
        "".to_string()
    }

    fn solve_part2(&mut self, _lines: &[String]) -> String {
        todo!()
    }
}
