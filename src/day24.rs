use ahash::{HashSet, HashSetExt};
use std::collections::BTreeMap;

type N = u16;
type Pos = [N; 2];

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
struct Storm {
    i: N,
    start: N,
}

#[derive(Debug, Clone)]
pub struct Field {
    width: N,
    height: N,

    start_x: N,
    end_x: N,

    up: Vec<Storm>,
    down: Vec<Storm>,
    left: Vec<Storm>,
    right: Vec<Storm>,
}

pub fn generator(s: &str) -> Field {
    let mut lines = s.lines();

    let width;
    let start_x = {
        let first_line = lines.next().unwrap();
        width = first_line.len() - 2;
        N::try_from(first_line.find('.').unwrap() - 1).unwrap()
    };
    let end_x = {
        let last_line = lines.next_back().unwrap();
        N::try_from(last_line.find('.').unwrap() - 1).unwrap()
    };
    let mut y = 0;
    let mut up = Vec::new();
    let mut down = Vec::new();
    let mut left = Vec::new();
    let mut right = Vec::new();
    for line in lines {
        let line = &line[1..line.len() - 1];
        for (x, ch) in line.bytes().enumerate() {
            let x = x as N;
            match ch {
                b'.' => continue,
                b'^' => up.push(Storm { i: x, start: y }),
                b'v' => down.push(Storm { i: x, start: y }),
                b'<' => left.push(Storm { i: y, start: x }),
                b'>' => right.push(Storm { i: y, start: x }),
                _ => panic!("Unknown map symbol {ch}"),
            }
        }
        y += 1;
    }

    up.sort_unstable();
    down.sort_unstable();
    left.sort_unstable();
    right.sort_unstable();

    Field {
        width: width.try_into().unwrap(),
        height: y,
        start_x,
        end_x,
        up,
        down,
        left,
        right,
    }
}

pub fn part_1(field: &Field) -> u16 {
    let turns = a_star(field, 1, false);

    turns
}

pub fn part_2(field: &Field) -> u16 {
    let first_leg = a_star(field, 1, false);
    let second_leg = a_star(field, first_leg + 1, true);
    let third_leg = a_star(field, second_leg + 1, false);

    third_leg
}

const fn heuristic(src: Pos, dst: Pos) -> N {
    src[0].abs_diff(dst[0]) + src[1].abs_diff(dst[1])
}

fn a_star(field: &Field, mut start_turn: u16, reverse: bool) -> u16 {
    let mut frontier = BTreeMap::<u16, Vec<(Pos, u16)>>::new();
    let mut visited = HashSet::with_capacity(1024);

    let (start, end) = {
        let start = [field.start_x, 0];
        let end = [field.end_x, field.height - 1];
        if reverse {
            (end, start)
        } else {
            (start, end)
        }
    };

    loop {
        while field.is_taken(start_turn, start) {
            start_turn += 1;
        }
        frontier.insert(0, vec![(start, start_turn)]);

        while let Some((cost_est, mut items)) = frontier.pop_first() {
            while let Some((current, current_turn)) = items.pop() {
                if current == end {
                    return current_turn + 1;
                }

                let mut best_estimate = u16::MAX;
                for diff in OFFSETS {
                    let Some(new_x) = current[0].checked_add_signed(diff[0]) else { continue };
                    if new_x >= field.width {
                        continue;
                    }
                    let Some(new_y) = current[1].checked_add_signed(diff[1]) else { continue };
                    if new_y >= field.height {
                        continue;
                    }
                    let new_pos = [new_x, new_y];
                    let new_turn = current_turn + 1;

                    if !visited.insert((new_pos, new_turn)) {
                        continue;
                    }
                    if field.is_taken(new_turn, new_pos) {
                        continue;
                    }
                    let new_estimate = new_turn + heuristic(new_pos, end);
                    frontier
                        .entry(new_estimate)
                        .or_default()
                        .push((new_pos, new_turn));
                    best_estimate = best_estimate.min(new_estimate);
                }

                // If we find a better estimate in the middle of a list of items,
                // add the list back, and start from there
                if best_estimate < cost_est && !items.is_empty() {
                    frontier
                        .entry(cost_est)
                        .or_default()
                        .extend_from_slice(&items);
                    break;
                }
            }
        }
        start_turn += 1;
    }
}

const OFFSETS: [[i16; 2]; 5] = [[0, 0], [-1, 0], [1, 0], [0, -1], [0, 1]];

impl Field {
    fn is_taken(&self, turn: u16, [x, y]: Pos) -> bool {
        let vert_turn_offset = turn % self.height;
        let horiz_turn_offset = turn % self.width;

        {
            let up_start = self.up.partition_point(|storm| storm.i < x);
            for storm in &self.up[up_start..] {
                if storm.i != x {
                    break;
                }
                if (storm.start + self.height - vert_turn_offset) % self.height == y {
                    return true;
                }
            }
        }
        {
            let down_start = self.down.partition_point(|storm| storm.i < x);
            for storm in &self.down[down_start..] {
                if storm.i != x {
                    break;
                }
                if (storm.start + vert_turn_offset) % self.height == y {
                    return true;
                }
            }
        }
        {
            let left_start = self.left.partition_point(|storm| storm.i < y);
            for storm in &self.left[left_start..] {
                if storm.i != y {
                    break;
                }
                if (storm.start + self.width - horiz_turn_offset) % self.width == x {
                    return true;
                }
            }
        }
        {
            let right_start = self.right.partition_point(|storm| storm.i < y);
            for storm in &self.right[right_start..] {
                if storm.i != y {
                    break;
                }
                if (storm.start + horiz_turn_offset) % self.width == x {
                    return true;
                }
            }
        }
        false
    }
}
