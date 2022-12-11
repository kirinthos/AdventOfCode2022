use num_enum::{IntoPrimitive, TryFromPrimitive};

use crate::Problem;

#[derive(IntoPrimitive, TryFromPrimitive, Debug, Clone, Copy)]
#[repr(u8)]
enum PlayResult {
    Lose = 0,
    Draw = 3,
    Win = 6,
}

#[derive(IntoPrimitive, TryFromPrimitive, Debug, Clone, Copy)]
#[repr(u8)]
enum Choice {
    Rock = 1,
    Paper,
    Scissors,
}

impl Choice {
    fn play(&self, other: &Choice) -> PlayResult {
        match (self, other) {
            (Choice::Rock, Choice::Rock)
            | (Choice::Paper, Choice::Paper)
            | (Choice::Scissors, Choice::Scissors) => PlayResult::Draw,

            (Choice::Rock, Choice::Paper)
            | (Choice::Paper, Choice::Scissors)
            | (Choice::Scissors, Choice::Rock) => PlayResult::Lose,

            (Choice::Rock, Choice::Scissors)
            | (Choice::Paper, Choice::Rock)
            | (Choice::Scissors, Choice::Paper) => PlayResult::Win,
        }
    }

    fn which_play(&self, result: &PlayResult) -> Choice {
        match (self, result) {
            (Choice::Scissors, PlayResult::Draw)
            | (Choice::Paper, PlayResult::Win)
            | (Choice::Rock, PlayResult::Lose) => Choice::Scissors,

            (Choice::Scissors, PlayResult::Win)
            | (Choice::Paper, PlayResult::Lose)
            | (Choice::Rock, PlayResult::Draw) => Choice::Rock,

            (Choice::Scissors, PlayResult::Lose)
            | (Choice::Paper, PlayResult::Draw)
            | (Choice::Rock, PlayResult::Win) => Choice::Paper,
        }
    }
}

pub struct Problem2;

impl Problem for Problem2 {
    fn solve_part1(&mut self, lines: &[String]) -> String {
        lines
            .iter()
            .map(|l| l.split_at(1))
            .map(|(v1, v2)| {
                (
                    Choice::try_from((v1.chars().next().unwrap() as u8 - b'A') + 1).unwrap(),
                    Choice::try_from((v2.chars().nth(1).unwrap() as u8 - b'X') + 1).unwrap(),
                )
            })
            .map(|(opp, me)| {
                let win: u8 = me.play(&opp).into();
                let me: u8 = me.into();
                (win + me) as i64
            })
            .sum::<i64>()
            .to_string()
    }

    fn solve_part2(&mut self, lines: &[String]) -> String {
        lines
            .iter()
            .map(|l| l.split_at(1))
            .map(|(v1, v2)| {
                (
                    Choice::try_from((v1.chars().next().unwrap() as u8 - b'A') + 1).unwrap(),
                    PlayResult::try_from((v2.chars().nth(1).unwrap() as u8 - b'X') * 3).unwrap(),
                )
            })
            .map(|(opp, result)| {
                let me: u8 = opp.which_play(&result).into();
                let result: u8 = result.into();
                (me + result) as i64
            })
            .sum::<i64>()
            .to_string()
    }
}
