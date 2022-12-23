use ahash::{HashMap, HashMapExt};
use bitvec::prelude::*;
use std::collections::hash_map::Entry;
use std::fmt;
use std::fmt::Write;

const COLUMNS: usize = 7;

#[derive(Default, Debug, Copy, Clone, PartialEq, Eq, Hash)]
#[repr(u8)]
enum Shape {
    #[default]
    Minus = 0,
    Plus,
    L,
    I,
    Square,
}

impl Shape {
    fn next(self) -> Self {
        match self {
            Shape::Minus => Shape::Plus,
            Shape::Plus => Shape::L,
            Shape::L => Shape::I,
            Shape::I => Shape::Square,
            Shape::Square => Shape::Minus,
        }
    }

    fn width(self) -> u32 {
        match self {
            Shape::Minus => 4,
            Shape::Plus => 3,
            Shape::L => 3,
            Shape::I => 1,
            Shape::Square => 2,
        }
    }

    const fn offsets(self) -> &'static [(u32, u32)] {
        match self {
            Shape::Minus => &[(0, 0), (1, 0), (2, 0), (3, 0)],
            Shape::Plus => &[(1, 0), (0, 1), (1, 1), (2, 1), (1, 2)],
            Shape::L => &[(0, 0), (1, 0), (2, 0), (2, 1), (2, 2)],
            Shape::I => &[(0, 0), (0, 1), (0, 2), (0, 3)],
            Shape::Square => &[(0, 0), (1, 0), (0, 1), (1, 1)],
        }
    }

    fn hit_test(self, x: u32, y: u32, board: &Board) -> bool {
        for &(dx, dy) in self.offsets() {
            if board.get(x + dx, y + dy) {
                return true;
            }
        }
        false
    }

    fn set(self, x: u32, y: u32, board: &mut Board) {
        for &(dx, dy) in self.offsets().iter().rev() {
            board.set(x + dx, y + dy)
        }
    }
}

#[derive(Default, Clone, PartialEq, Eq, Hash)]
pub struct Board {
    bits: BitVec,
}

fn idx(x: u32, y: u32) -> usize {
    let x: usize = x.try_into().unwrap();
    let y: usize = y.try_into().unwrap();
    debug_assert!(x < COLUMNS);

    y * COLUMNS + x
}

impl Board {
    fn get(&self, x: u32, y: u32) -> bool {
        self.bits.get(idx(x, y)).map_or(false, |bit| *bit)
    }

    fn set(&mut self, x: u32, y: u32) {
        let idx = idx(x, y);
        if idx >= self.bits.len() {
            self.bits.resize(idx + 1, false);
        }
        self.bits.set(idx, true);
    }

    fn height(&self) -> u32 {
        ((self.bits.len() + (COLUMNS - 1)) / COLUMNS)
            .try_into()
            .unwrap()
    }
}

pub fn generator(s: &str) -> &str {
    s.trim_end()
}

#[derive(Debug, Clone, Default, PartialEq, Eq, Hash)]
struct Game {
    board: Board,
    piece: Shape,
    jet_idx: u32,
}

#[derive(Copy, Clone)]
struct Jets<'a>(&'a [u8]);

impl Jets<'_> {
    fn next_x(&self, idx: &mut u32) -> i8 {
        let i = *idx as usize;
        if i + 1 == self.0.len() {
            *idx = 0;
        } else {
            *idx += 1;
        }
        if self.0[i] == b'<' {
            -1
        } else {
            debug_assert_eq!(self.0[i], b'>');
            1
        }
    }
}

impl Game {
    fn drop(&mut self, jets: Jets) {
        let piece = self.piece;
        self.piece = piece.next();

        let mut x = 2u32;

        for _ in 0..3 {
            let new_x = x.saturating_add_signed(jets.next_x(&mut self.jet_idx).into());
            x = new_x.min(COLUMNS as u32 - piece.width());
            debug_assert!(!piece.hit_test(x, self.board.height(), &self.board));
        }

        let mut move_wind = |x: &mut u32, y: u32| {
            let new_x = x
                .checked_add_signed(jets.next_x(&mut self.jet_idx).into())
                .filter(|&i| i <= COLUMNS as u32 - piece.width());
            if let Some(new_x) = new_x {
                if !piece.hit_test(new_x, y, &self.board) {
                    *x = new_x;
                }
            }
        };
        for y in (1..=self.board.height()).rev() {
            move_wind(&mut x, y);

            if piece.hit_test(x, y - 1, &self.board) {
                piece.set(x, y, &mut self.board);
                return;
            }
        }

        move_wind(&mut x, 0);
        piece.set(x, 0, &mut self.board);
    }
}

pub fn part_1(s: &str) -> u32 {
    let mut game = Game::default();
    let jets = Jets(s.as_bytes());

    for _ in 0..2022 {
        game.drop(jets);
    }

    game.board.height()
}

pub fn part_2(s: &str) -> u64 {
    const ROCK_COUNT: u64 = 1_000_000_000_000;

    let mut game = Game::default();
    let jets = Jets(s.as_bytes());

    for _ in 0..500 {
        game.drop(jets);
    }

    let mut games = HashMap::with_capacity(1024);

    let mut i = 500;
    let (loop_len, loop_height) = loop {
        match games.entry((
            game.piece,
            game.jet_idx,
            game.board.bits[game.board.bits.len() - (100 * COLUMNS)..].to_bitvec(),
        )) {
            Entry::Occupied(e) => {
                let &(old_i, old_height) = e.get();
                break (i - old_i, game.board.height() - old_height);
            }
            Entry::Vacant(e) => {
                e.insert((i, game.board.height()));
            }
        }
        game.drop(jets);
        i += 1;
    };

    let extra_after_loops = (ROCK_COUNT - i) % loop_len;
    for _ in 0..extra_after_loops {
        game.drop(jets);
    }

    game.board.height() as u64 + ((ROCK_COUNT - i) / loop_len) * u64::from(loop_height)
}

impl fmt::Debug for Board {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // let min_y = self.height().saturating_sub(8);
        let min_y = 0;
        for y in (min_y..self.height()).rev() {
            f.write_char('\n')?;
            for x in 0..COLUMNS {
                let ch = if self.get(x as u32, y) { '#' } else { '.' };
                f.write_char(ch)?;
            }
        }
        Ok(())
    }
}

super::day_test! {demo_1 == 3068}
super::day_test! {demo_2 == 1514285714288}
super::day_test! {part_1 == 3149}
super::day_test! {part_2 == 1553982300884}
