use bitvec::bitbox;
use bitvec::prelude::*;
use std::fmt::Write;
use std::{fmt, iter};

fn lines(s: &str) -> impl Iterator<Item = impl Iterator<Item = (i32, i32)> + '_> + '_ {
    s.lines().map(|line| {
        line.split(" -> ").map(|pair| {
            let (x, y) = pair.split_once(',').unwrap();
            let x = x.parse().unwrap();
            let y = y.parse().unwrap();
            (x, y)
        })
    })
}

#[derive(Clone)]
pub struct Map {
    x_start: i32,
    width: i32,
    filled: BitBox,
}

impl Map {
    fn drop(&mut self) -> (i32, i32) {
        let mut point = (500, 0);
        'outer: loop {
            for (dx, dy) in [(0, 1), (-1, 1), (1, 1)] {
                if let Some(false) = self.get(point.0 + dx, point.1 + dy) {
                    point.0 += dx;
                    point.1 += dy;
                    continue 'outer;
                }
            }
            self.set(point.0, point.1);
            return point;
        }
    }

    fn get(&self, x: i32, y: i32) -> Option<bool> {
        let idx = self.idx(x, y)?;
        Some(self.filled[idx])
    }

    fn set(&mut self, x: i32, y: i32) {
        let idx = self.idx(x, y).unwrap();
        self.filled.set(idx, true);
    }

    fn idx(&self, x: i32, y: i32) -> Option<usize> {
        if x < self.x_start || x >= self.x_start + self.width || y < 0 || y >= self.height() {
            return None;
        }
        let x = x - self.x_start;
        let idx = (x + self.width * y) as usize;
        Some(idx)
    }

    fn height(&self) -> i32 {
        self.filled.len() as i32 / self.width
    }
}

pub fn generator(s: &str) -> Map {
    let mut min_x = i32::MAX;
    let mut max_x = 0;
    let mut max_y = 0;
    iter::once((500, 0))
        .chain(lines(s).flatten())
        .for_each(|(x, y)| {
            min_x = min_x.min(x);
            max_x = max_x.max(x + 1);
            max_y = max_y.max(y + 1);
        });

    let height = max_y + 2;
    min_x = min_x.min(500 - height - 1);
    max_x = max_x.max(500 + height + 1);

    let width = max_x - min_x;

    let mut map = Map {
        x_start: min_x,
        width,
        filled: bitbox![0; (width * height) as usize],
    };

    for mut line in lines(s) {
        let mut last_point = line.next().unwrap();
        map.set(last_point.0, last_point.1);
        for next_point in line {
            while last_point != next_point {
                last_point.0 += (next_point.0 - last_point.0).signum();
                last_point.1 += (next_point.1 - last_point.1).signum();
                map.set(last_point.0, last_point.1);
            }
        }
    }

    for x in 0..width {
        map.set(map.x_start + x, height - 1);
    }
    map
}

pub fn part_1(map: &Map) -> u32 {
    let mut map = map.clone();

    let mut count = 0;
    while map.drop().1 < map.height() - 2 {
        count += 1;
    }

    count
}

pub fn part_2(orig_map: &Map) -> u32 {
    let mut map = Map {
        x_start: orig_map.x_start,
        width: orig_map.width,
        filled: bitbox![0; orig_map.filled.len()],
    };

    map.set(500, 0);
    let mut count = 1;

    for y in 0..map.height() - 1 {
        let triangle_width = y * 2 + 1;
        let start_x = 500 - triangle_width / 2;
        let end_x = 500 + triangle_width / 2;
        for x in start_x..=end_x {
            if !map.get(x, y).unwrap() {
                continue;
            }
            // propagate down in a triangle
            for (dx, dy) in [(-1, 1), (0, 1), (1, 1)] {
                let (new_x, new_y) = (x + dx, y + dy);
                if orig_map.get(new_x, new_y).unwrap() {
                    continue;
                }
                if !map.get(new_x, new_y).unwrap() {
                    count += 1;
                    map.set(new_x, new_y);
                }
            }
        }
    }
    count
}

impl fmt::Debug for Map {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_char('\n')?;
        for y in 0..self.height() {
            for x in 0..self.width {
                let x = self.x_start + x;
                let ch = if self.get(x, y).unwrap() { '#' } else { '.' };
                f.write_char(ch)?;
            }
            f.write_char('\n')?;
        }
        Ok(())
    }
}

super::day_test! {demo_1 == 24}
super::day_test! {demo_2 == 93}
super::day_test! {part_1 == 763}
super::day_test! {part_2 == 23921}
