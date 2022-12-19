use crate::{point::Point, Problem};

type Board = [[bool; 7]; 20000];

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
    let mut map: Board = [[false; 7]; 20000];
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

                    if highest > 3000 {
                        shift_map(&mut map, 1000);
                        highest -= 1000;
                        highest_base += 1000;
                    }

                    break;
                }
            }
        }
    }

    (highest, heights)
}

fn shift_map(map: &mut Board, n: usize) {
    let mut b = [[false; 7]; 20000];
    b[0..n].copy_from_slice(&map[n..(n + n)]);
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
        let n = 600000;
        let heights = solve(lines[0].as_str(), n).1;

        for start in 0..heights.len() {
            let start_height = heights[start] as i64;
            for period_size in 3..((n as usize - start) as usize / 2) {
                if heights
                    .iter()
                    .skip(start)
                    .take(period_size)
                    .zip(heights.iter().skip(start + period_size).take(period_size))
                    .all(|(one, two)| *one == (two - start_height as i32))
                {
                    println!(
                        "start: {}, period: {}\nstart height: {}, period height: {}",
                        start,
                        period_size,
                        heights[start],
                        heights[start + period_size]
                    );
                    let mut turns = 1_000_000_000_000_i64;

                    // start
                    turns -= start as i64;

                    // as many periods as we can fit
                    let num_periods = turns / period_size as i64;
                    let remaining = turns % period_size as i64;

                    let period_height =
                        num_periods * (heights[start + period_size] as i64 - start_height);
                    let remaining_height =
                        heights[start + remaining as usize] as i64 - start_height as i64;

                    println!("num_periods: {num_periods}, remaining: {remaining}");
                    println!(
                        "period height: {period_height}, remaining height: {remaining_height}"
                    );

                    return (heights[start - 1] as i64 + period_height + remaining_height)
                        .to_string();
                }
            }
        }

        "".to_string()
    }
}
