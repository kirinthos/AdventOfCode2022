use crate::Problem;

pub struct Problem1;
impl Problem for Problem1 {
    fn solve_part1(&mut self, lines: &[String]) -> String {
        let packs = self.read_packs(lines);
        packs
            .iter()
            .map(|pack| pack.iter().sum::<i64>())
            .max()
            .unwrap()
            .to_string()
    }

    fn solve_part2(&mut self, lines: &[String]) -> String {
        let packs = self.read_packs(lines);
        let mut sums: Vec<_> = packs.iter().map(|pack| pack.iter().sum()).collect();
        sums.sort_unstable();
        sums.iter().rev().take(3).sum::<i64>().to_string()
    }
}

impl Problem1 {
    fn read_packs(&self, lines: &[String]) -> Vec<Vec<i64>> {
        lines
            .to_vec()
            .split(|s| matches!(s.as_str(), ""))
            .map(|ss| ss.iter().map(|s| s.parse::<i64>().unwrap()).collect())
            .collect()
    }
}
