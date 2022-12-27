use ahash::{HashMap, HashMapExt, HashSet, HashSetExt};
use bitvec::BitArr;
use std::collections::BTreeMap;

type N = u16;
type Pos = [N; 2];

#[derive(Debug, Copy, Clone, Default)]
struct StormLine(BitArr!(for 128));

#[derive(Debug, Clone)]
pub struct Field {
    width: N,
    height: N,

    start_x: N,
    end_x: N,

    up: HashMap<N, StormLine>,
    down: HashMap<N, StormLine>,
    left: HashMap<N, StormLine>,
    right: HashMap<N, StormLine>,
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
    let mut up = HashMap::<N, StormLine>::new();
    let mut down = HashMap::<N, StormLine>::new();
    let mut left = HashMap::<N, StormLine>::new();
    let mut right = HashMap::<N, StormLine>::new();
    for line in lines {
        let line = &line[1..line.len() - 1];
        for (x, ch) in line.bytes().enumerate() {
            let x = x as N;
            match ch {
                b'.' => continue,
                b'^' => up.entry(x).or_default().0.set(y as usize, true),
                b'v' => down.entry(x).or_default().0.set(y as usize, true),
                b'<' => left.entry(y).or_default().0.set(x as usize, true),
                b'>' => right.entry(y).or_default().0.set(x as usize, true),
                _ => panic!("Unknown map symbol {ch}"),
            }
        }
        y += 1;
    }

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
            let orig_y = (y + vert_turn_offset) % self.height;
            if self
                .up
                .get(&x)
                .map_or(false, |storms| storms.0[orig_y as usize])
            {
                return true;
            }
        }
        {
            let orig_y = (y + self.height - vert_turn_offset) % self.height;
            if self
                .down
                .get(&x)
                .map_or(false, |storms| storms.0[orig_y as usize])
            {
                return true;
            }
        }
        {
            let orig_x = (x + horiz_turn_offset) % self.width;
            if self
                .left
                .get(&y)
                .map_or(false, |storms| storms.0[orig_x as usize])
            {
                return true;
            }
        }
        {
            let orig_x = (x + self.width - horiz_turn_offset) % self.width;
            if self
                .right
                .get(&y)
                .map_or(false, |storms| storms.0[orig_x as usize])
            {
                return true;
            }
        }
        false
    }
}

super::day_test! {demo_1 == 18}
super::day_test! {demo_2 == 54}
super::day_test! {part_1 == 283}
super::day_test! {part_2 == 883}
