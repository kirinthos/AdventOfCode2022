use crate::{point::Point, Problem};

type Board = [[bool; 7]; 40000];

enum Direction {
    Left,
    Right,
    Down,
}

fn solve(moves: &str, count: i64) -> (i32, Vec<i32>) {
    let pieces = [
        vec![
            Point::new(0, 0),
            Point::new(1, 0),
            Point::new(2, 0),
            Point::new(3, 0),
        ],
        vec![
            Point::new(1, 0),
            Point::new(0, 1),
            Point::new(1, 1),
            Point::new(2, 1),
            Point::new(1, 2),
        ],
        vec![
            Point::new(0, 0),
            Point::new(1, 0),
            Point::new(2, 0),
            Point::new(2, 1),
            Point::new(2, 2),
        ],
        vec![
            Point::new(0, 0),
            Point::new(0, 1),
            Point::new(0, 2),
            Point::new(0, 3),
        ],
        vec![
            Point::new(0, 0),
            Point::new(1, 0),
            Point::new(0, 1),
            Point::new(1, 1),
        ],
    ];

    let mut moves = moves.chars().cycle();
    let mut next_piece = 0;
    let mut map: Board = [[false; 7]; 40000];
    let mut highest = 0;
    let mut highest_base = 0;
    let mut heights = Vec::new();

    for _ in 0..count {
        //2022 {
        // place piece
        let mut piece = pieces[next_piece].clone();
        next_piece = (next_piece + 1) % pieces.len();

        piece.iter_mut().for_each(|p| {
            p.shift(2, highest + 3);
        });

        // run moves until piece sets
        loop {
            match moves.next().unwrap() {
                '<' => {
                    if can_move(&map, &piece, Direction::Left) {
                        piece.iter_mut().for_each(|p| p.shift(-1, 0))
                    }
                }
                '>' => {
                    if can_move(&map, &piece, Direction::Right) {
                        piece.iter_mut().for_each(|p| p.shift(1, 0))
                    }
                }
                _ => panic!("bad character"),
            }

            match can_move(&map, &piece, Direction::Down) {
                true => piece.iter_mut().for_each(|p| p.shift(0, -1)),
                false => {
                    for p in piece.iter() {
                        map[p.y() as usize][p.x() as usize] = true;
                        highest = highest.max(p.y() + 1);
                    }

                    heights.push(highest + highest_base);

                    /*
                    if highest > 6000 {
                        shift_map(&mut map, 1000);
                        highest -= 1000;
                        highest_base += 1000;
                    }
                    */

                    break;
                }
            }
        }
    }

    (highest, heights)
}

fn shift_map(map: &mut Board, n: usize) {
    let mut b = [[false; 7]; 40000];
    b[0..(40000 - n)].copy_from_slice(&map[n..]);
    *map = b
}

fn can_move(map: &Board, piece: &[Point], direction: Direction) -> bool {
    match direction {
        Direction::Left => piece.iter().all(|p| {
            let moved = *p - Point::new(1, 0);
            moved.x() >= 0 && !map[moved.y() as usize][moved.x() as usize]
        }),
        Direction::Right => piece.iter().all(|p| {
            let moved = *p + Point::new(1, 0);
            moved.x() < 7 && !map[moved.y() as usize][moved.x() as usize]
        }),
        Direction::Down => piece.iter().all(|p| {
            let moved = *p - Point::new(0, 1);
            moved.y() >= 0 && !map[moved.y() as usize][moved.x() as usize]
        }),
    }
}

#[allow(dead_code)]
fn print_map(map: &Board, piece: Option<&[Point]>, highest: usize) {
    for (y, row) in map.iter().enumerate().take(highest + 1).rev() {
        for (x, c) in row.iter().enumerate() {
            if piece.map_or(false, |p| {
                p.iter().any(|p| p.y() == y as i32 && p.x() == x as i32)
            }) {
                print!("@");
            } else {
                print!(
                    "{}",
                    match c {
                        true => '#',
                        false => '.',
                    }
                );
            }
        }
        println!();
    }
    println!();
}

pub struct Problem17;
impl Problem for Problem17 {
    fn solve_part1(&mut self, lines: &[String]) -> String {
        solve(lines[0].as_str(), 2022).0.to_string()
    }

    fn solve_part2(&mut self, lines: &[String]) -> String {
        let n = 20000;
        let heights = solve(lines[0].as_str(), n).1;
        let diffs = heights
            .iter()
            .zip(heights.iter().skip(1))
            .map(|(one, two)| two - one)
            .collect::<Vec<_>>();

        for start in 1..heights.len() {
            for period_size in 3..5000 {
                let p1_end = start + period_size;
                let p2_end = p1_end + period_size;
                let p3_end = p2_end + period_size;

                if p2_end > diffs.len() {
                    panic!("nothing found");
                }

                let p1 = &diffs[start..p1_end];
                let p2 = &diffs[p1_end..p2_end];
                let p3 = &diffs[p2_end..p3_end];
                if p1 == p2 && p2 == p3 {
                    let start_height = diffs[..start].iter().sum::<i32>() as i64;
                    let period_height = p1.iter().sum::<i32>() as i64;

                    let turns = 1_000_000_000_000_i64 - start as i64;
                    let periods = turns / period_size as i64;
                    let remaining = turns % period_size as i64;

                    let height = start_height
                        + periods * period_height
                        + diffs[p1_end..(p1_end + remaining as usize)]
                            .iter()
                            .sum::<i32>() as i64;

                    return height.to_string();
                }
            }
        }

        "".to_string()
    }
}
