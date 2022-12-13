use ahash::{HashMap, HashMapExt};
use std::collections::BinaryHeap;
use std::{cmp, iter};

use crate::grid::NlGrid;

type Pos = (i32, i32);

const fn height(h: u8) -> u8 {
    match h {
        b'S' => b'a',
        b'E' => b'z',
        _ => h,
    }
}

const fn can_climb(h1: u8, h2: u8) -> bool {
    height(h1) + 1 >= height(h2)
}

#[derive(Debug)]
pub struct Map<'a> {
    grid: NlGrid<'a>,
    start: Pos,
    end: Pos,
}

pub fn generator(s: &str) -> Map {
    let grid = NlGrid::new(s);

    let start = grid.position_of(b'S').unwrap();
    let end = grid.position_of(b'E').unwrap();

    Map { grid, start, end }
}

pub fn part_1(map: &Map) -> u32 {
    let mut came_from = HashMap::with_capacity(1024);
    let mut cost_so_far = HashMap::with_capacity(1024);
    a_star(
        map.grid,
        iter::once(map.start),
        map.end,
        &mut came_from,
        &mut cost_so_far,
    );
    cost_so_far[&map.end]
}

pub fn part_2(map: &Map) -> u32 {
    let mut came_from = HashMap::with_capacity(1024);
    let mut cost_so_far = HashMap::with_capacity(1024);
    a_star(
        map.grid,
        map.grid.multi_position(b'a'),
        map.end,
        &mut came_from,
        &mut cost_so_far,
    );
    cost_so_far[&map.end]
}

const fn heuristic(src: Pos, dst: Pos) -> u32 {
    src.0.abs_diff(dst.0) + src.1.abs_diff(dst.1)
}

fn a_star(
    grid: NlGrid,
    starts: impl Iterator<Item = Pos>,
    end: Pos,
    came_from: &mut HashMap<Pos, Pos>,
    cost_so_far: &mut HashMap<Pos, u32>,
) {
    const DIFFS: [(i32, i32); 4] = [(1, 0), (-1, 0), (0, 1), (0, -1)];
    let mut frontier = BinaryHeap::with_capacity(1024);

    for start in starts {
        let start_height = grid.get(start.0, start.1).unwrap();
        frontier.push((cmp::Reverse(0), (start, start_height)));
        came_from.insert(start, start);
        cost_so_far.insert(start, 0);
    }

    while let Some((_, (current, current_height))) = frontier.pop() {
        if current == end {
            break;
        }

        let current_cost = cost_so_far[&current];
        for diff in DIFFS {
            let new_pos = (current.0 + diff.0, current.1 + diff.1);
            let new_cost = current_cost + 1;

            let Some(new_height) = grid.get(new_pos.0, new_pos.1) else { continue };
            if !can_climb(current_height, new_height) {
                continue;
            }
            if cost_so_far
                .get(&new_pos)
                .map_or(true, |&cost| new_cost < cost)
            {
                cost_so_far.insert(new_pos, new_cost);
                let priority = new_cost + heuristic(new_pos, end);
                frontier.push((cmp::Reverse(priority), (new_pos, new_height)));
                came_from.insert(new_pos, current);
            }
        }
    }
}

super::day_test! {demo_1 == 31}
super::day_test! {part_1 == 440}
super::day_test! {demo_2 == 29}
super::day_test! {part_2 == 439}
