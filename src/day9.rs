use ahash::{HashSet, HashSetExt};
use arrayvec::ArrayVec;
use std::cmp;

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
    fn move_toward(&mut self, other: Self) -> bool;
    fn move_to(&mut self, dir: Dir);
}

impl MoveVect for Vect {
    // return if move happened
    fn move_toward(&mut self, other: Self) -> bool {
        let x_diff = other.0 - self.0;
        let y_diff = other.1 - self.1;
        let dist = cmp::max(x_diff.abs(), y_diff.abs());
        if dist <= 1 {
            return false;
        }
        self.0 += x_diff.signum();
        self.1 += y_diff.signum();
        true
    }

    fn move_to(&mut self, dir: Dir) {
        let diff = dir.to_vect();
        self.0 += diff.0;
        self.1 += diff.1;
    }
}

#[derive(Copy, Clone)]
struct Section {
    ty: Type,
    start: Vect,
    count: u8,
}

impl Section {
    fn end(self) -> Vect {
        let count = i32::from(self.count);
        let (dx, dy) = match self.ty {
            Type::Together => (0, 0),
            Type::StraightLeft => (-count, 0),
            Type::StraightRight => (count, 0),
            Type::StraightUp => (0, count),
            Type::StraightDown => (0, -count),
            Type::DiagonalRightDown => (count, -count),
            Type::DiagonalLeftDown => (-count, -count),
            Type::DiagonalRightUp => (count, count),
            Type::DiagonalLeftUp => (-count, count),
        };
        let (x, y) = self.start;
        (x + dx, y + dy)
    }

    fn move_to(mut self, dir: Dir, count: i32) -> Option<(Self, Option<Self>)> {
        assert!(count > 0);

        let dir_vect = dir.to_vect();
        let ty_vect = self.ty.going_vect();
        if dir_vect == ty_vect {
            self.start.0 += dir_vect.0 * count;
            self.start.1 += dir_vect.1 * count;
            return Some((self, None));
        }
        if ty_vect == (0, 0) {
            let new = Self {
                ty: dir.into(),
                start: (
                    self.start.0 + dir_vect.0 * count,
                    self.start.1 + dir_vect.1 * count,
                ),
                count: cmp::min(self.count, count.try_into().unwrap_or(u8::MAX)),
            };
            let remainder = (count < i32::from(self.count)).then_some(Self {
                ty: Type::Together,
                start: self.start,
                count: count as u8,
            });
            return Some((new, remainder));
        }
        None
    }
}

#[derive(Copy, Clone)]
enum Type {
    Together,
    StraightLeft,
    StraightRight,
    StraightUp,
    StraightDown,
    DiagonalRightDown,
    DiagonalLeftDown,
    DiagonalRightUp,
    DiagonalLeftUp,
}

impl Type {
    fn to_vect(self) -> Vect {
        match self {
            Type::Together => (0, 0),
            Type::StraightLeft => (-1, 0),
            Type::StraightRight => (1, 0),
            Type::StraightUp => (0, 1),
            Type::StraightDown => (0, -1),
            Type::DiagonalRightDown => (1, -1),
            Type::DiagonalLeftDown => (-1, -1),
            Type::DiagonalRightUp => (1, 1),
            Type::DiagonalLeftUp => (-1, 1),
        }
    }

    fn going_vect(self) -> Vect {
        let (dx, dy) = self.to_vect();
        (-dx, -dy)
    }
}

impl From<Dir> for Type {
    fn from(dir: Dir) -> Self {
        match dir {
            Dir::Up => Self::StraightUp,
            Dir::Down => Self::StraightDown,
            Dir::Left => Self::StraightLeft,
            Dir::Right => Self::StraightRight,
        }
    }
}

struct Rope<const KNOTS: usize> {
    sections: ArrayVec<Section, KNOTS>,
}

impl<const KNOTS: usize> Rope<KNOTS> {
    fn new() -> Self {
        let mut sections = ArrayVec::new();
        sections.push(Section {
            ty: Type::Together,
            start: (0, 0),
            count: KNOTS as _,
        });
        Self { sections }
    }

    fn check(&self) {
        #[cfg(debug_assertions)]
        {
            debug_assert!(!self.sections.is_empty());
            let total_len: usize = self.sections.iter().map(|s| usize::from(s.count)).sum();
            debug_assert_eq!(total_len, KNOTS);
        }
    }

    fn move_to(&mut self, dir: Dir) {
        match dir {
            Dir::Up => {}
            Dir::Down => {}
            Dir::Left => {}
            Dir::Right => {}
        }
    }
}

fn count_tail_positions<const KNOTS: usize>(moves: &[Move]) -> usize {
    todo!()
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
