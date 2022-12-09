use std::cmp;
use std::collections::HashSet;

type Dist = i32;

type Vect = (Dist, Dist);

#[repr(u8)]
#[derive(Debug, Copy, Clone)]
enum Dir {
    Up = b'U',
    Down = b'D',
    Left = b'L',
    Right = b'R',
}

impl Dir {
    fn to_vect(self) -> Vect {
        match self {
            Self::Up => (0, 1),
            Self::Down => (0, -1),
            Self::Left => (-1, 0),
            Self::Right => (1, 0),
        }
    }
}

#[derive(Debug)]
pub struct Move {
    dir: Dir,
    count: u32,
}

pub fn generator(s: &str) -> Vec<Move> {
    let mut result = Vec::with_capacity(2048);
    for line in s.lines() {
        let dir = line.as_bytes()[0];
        // Direction is one byte, and skip the space
        let count_str = &line[2..];
        let count = count_str.parse().unwrap();

        let dir = match dir {
            b'U' => Dir::Up,
            b'D' => Dir::Down,
            b'L' => Dir::Left,
            b'R' => Dir::Right,
            _ => panic!("unexpected direction {}", dir as char),
        };
        result.push(Move { dir, count });
    }
    result
}

trait MoveVect {
    fn move_toward(&mut self, other: Self);
    fn move_to(&mut self, dir: Dir);
}

impl MoveVect for Vect {
    fn move_toward(&mut self, other: Self) {
        let x_diff = other.0 - self.0;
        let y_diff = other.1 - self.1;
        let dist = cmp::max(x_diff.abs(), y_diff.abs());
        if dist <= 1 {
            return;
        }
        self.0 += x_diff.signum();
        self.1 += y_diff.signum();
    }

    fn move_to(&mut self, dir: Dir) {
        let diff = dir.to_vect();
        self.0 += diff.0;
        self.1 += diff.1;
    }
}

fn count_tail_positions<const KNOTS: usize>(moves: &[Move]) -> usize {
    let mut knots = [(0, 0); KNOTS];

    let mut tail_positions = HashSet::with_capacity(2048);
    tail_positions.insert((0, 0));
    for m in moves {
        for _ in 0..m.count {
            knots[0].move_to(m.dir);
            for i in 1..KNOTS {
                knots[i].move_toward(knots[i - 1]);
            }
            tail_positions.insert(*knots.last().unwrap());
        }
    }
    tail_positions.len()
}

pub fn part_1(moves: &[Move]) -> usize {
    count_tail_positions::<2>(moves)
}

pub fn part_2(moves: &[Move]) -> usize {
    count_tail_positions::<10>(moves)
}

super::day_test! {demo_1 == 13}
super::day_test! {demo_2 == 1}
super::day_test! {part_1 == 6498}
super::day_test! {part_2 == 2531}

#[test]
fn demo2_2() {
    let input = "R 5\nU 8\nL 8\nD 3\nR 17\nD 10\nL 25\nU 20\n";
    let input = generator(input);
    assert_eq!(part_2(&input), 36);
}
