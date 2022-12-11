use crate::Problem;

pub struct Problem4;
impl Problem for Problem4 {
    fn solve_part1(&mut self, lines: &[String]) -> String {
        lines
            .iter()
            .map(|l| l.split_once(',').unwrap())
            .map(|(one, two)| (one.split_once('-').unwrap(), two.split_once('-').unwrap()))
            .filter(|((s1, e1), (s2, e2))| {
                let e = (e2.parse::<i64>().unwrap() - e1.parse::<i64>().unwrap()).signum();
                let s = (s2.parse::<i64>().unwrap() - s1.parse::<i64>().unwrap()).signum();
                // when 0, it's always contained, otherwise the difference between start and end
                // needs to be opposite sign
                matches!((s, e), (0, _) | (_, 0) | (-1, 1) | (1, -1))
            })
            .count()
            .to_string()
    }

    fn solve_part2(&mut self, lines: &[String]) -> String {
        lines
            .iter()
            .map(|l| l.split_once(',').unwrap())
            .map(|(one, two)| (one.split_once('-').unwrap(), two.split_once('-').unwrap()))
            .filter(|((s1, e1), (s2, e2))| {
                let e1 = e1.parse::<i64>().unwrap();
                let e2 = e2.parse::<i64>().unwrap();
                let s1 = s1.parse::<i64>().unwrap();
                let s2 = s2.parse::<i64>().unwrap();
                s1 <= e2 && e1 >= s2
            })
            .count()
            .to_string()
    }
}
