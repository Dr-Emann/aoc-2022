use bitvec::bitbox;
use std::cmp;

#[derive(Debug)]
pub struct Grid<'a> {
    width: usize,
    bytes: &'a [u8],
}

impl Grid<'_> {
    fn width(&self) -> usize {
        self.width
    }

    fn height(&self) -> usize {
        self.bytes.len() / self.width
    }

    fn get(&self, x: usize, y: usize) -> u8 {
        self.bytes[x + y * (self.width + 1)]
    }
}

pub fn generator(s: &str) -> Grid {
    let width = s.lines().next().unwrap().len();
    Grid {
        width,
        bytes: s.trim_end().as_bytes(),
    }
}

pub fn part_1(grid: &Grid) -> usize {
    let width = grid.width();
    let height = grid.height();

    let pos = |x: usize, y: usize| y * width + x;
    let mut visible = bitbox![0; width * height];
    let mut max_seen = vec![0u8; cmp::max(width, height)];

    // top
    for y in 0..height {
        for x in 0..width {
            let value = grid.get(x, y);
            if value > max_seen[x] {
                visible.set(pos(x, y), true);
                max_seen[x] = value;
            }
        }
    }

    max_seen.fill(0);
    // bottom
    for y in (0..height).rev() {
        for x in 0..width {
            let value = grid.get(x, y);
            if value > max_seen[x] {
                visible.set(pos(x, y), true);
                max_seen[x] = value;
            }
        }
    }

    max_seen.fill(0);
    // left
    for x in 0..width {
        for y in 0..height {
            let value = grid.get(x, y);
            if value > max_seen[y] {
                visible.set(pos(x, y), true);
                max_seen[y] = value;
            }
        }
    }

    max_seen.fill(0);
    // right
    for x in (0..width).rev() {
        for y in 0..height {
            let value = grid.get(x, y);
            if value > max_seen[y] {
                visible.set(pos(x, y), true);
                max_seen[y] = value;
            }
        }
    }

    visible.count_ones()
}

pub fn part_2(grid: &Grid) -> u32 {
    let width = grid.width();
    let height = grid.height();

    let pos = |x: usize, y: usize| y * width + x;
    let mut scores = vec![1u32; width * height];

    let mut seen_pos = vec![[0; 10]; cmp::max(width, height)];

    // top
    for y in 0..height {
        for x in 0..width {
            let tree_height = grid.get(x, y) - b'0';
            let distance = y - seen_pos[x][usize::from(tree_height)];
            scores[pos(x, y)] *= distance as u32;
            seen_pos[x][..=usize::from(tree_height)].fill(y);
        }
    }

    seen_pos.fill([height - 1; 10]);
    // bottom
    for y in (0..height).rev() {
        for x in 0..width {
            let tree_height = grid.get(x, y) - b'0';
            let distance = seen_pos[x][usize::from(tree_height)] - y;
            scores[pos(x, y)] *= distance as u32;
            seen_pos[x][..=usize::from(tree_height)].fill(y);
        }
    }

    seen_pos.fill([0; 10]);
    // left
    for x in 0..width {
        for y in 0..height {
            let tree_height = grid.get(x, y) - b'0';
            let distance = x - seen_pos[y][usize::from(tree_height)];
            scores[pos(x, y)] *= distance as u32;
            seen_pos[y][..=usize::from(tree_height)].fill(x);
        }
    }
    seen_pos.fill([width - 1; 10]);
    // left
    for x in (0..width).rev() {
        for y in 0..height {
            let tree_height = grid.get(x, y) - b'0';
            let distance = seen_pos[y][usize::from(tree_height)] - x;
            scores[pos(x, y)] *= distance as u32;
            seen_pos[y][..=usize::from(tree_height)].fill(x);
        }
    }

    scores.iter().copied().max().unwrap()
}

super::day_test! {demo_1 == 21}
super::day_test! {demo_2 == 8}
super::day_test! {part_1 == 1792}
super::day_test! {part_2 == 334880}
