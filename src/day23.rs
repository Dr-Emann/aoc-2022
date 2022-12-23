use ahash::{HashMap, HashMapExt, HashSet, HashSetExt};
use std::collections::hash_map::Entry;

type Pos = (i32, i32);

pub fn generator(s: &str) -> HashSet<Pos> {
    let mut result = HashSet::with_capacity(1024);
    for (y, line) in s.lines().enumerate() {
        for (x, ch) in line.bytes().enumerate() {
            if ch == b'#' {
                result.insert((x as i32, 0 - y as i32));
            }
        }
    }
    result
}

const CHECK_POSITIONS: [[Pos; 3]; 4] = [
    [(0, 1), (-1, 1), (1, 1)],
    [(0, -1), (-1, -1), (1, -1)],
    [(-1, 0), (-1, -1), (-1, 1)],
    [(1, 0), (1, -1), (1, 1)],
];

pub fn part_1(positions: &HashSet<Pos>) -> i32 {
    let mut check_pos_start = 0;
    let mut proposals = HashMap::with_capacity(positions.len());
    let mut conflicts = HashSet::with_capacity(positions.len());
    let mut positions = positions.clone();

    for _ in 0..10 {
        proposals.clear();
        conflicts.clear();
        for &(x, y) in &positions {
            let mut has_neighbor = false;
            let mut proposed_direction = None;
            'outer: for direction in CHECK_POSITIONS[check_pos_start..]
                .iter()
                .chain(&CHECK_POSITIONS[..check_pos_start])
            {
                for &(dx, dy) in direction {
                    if positions.contains(&(x + dx, y + dy)) {
                        has_neighbor = true;
                        if proposed_direction.is_some() {
                            break 'outer;
                        } else {
                            continue 'outer;
                        }
                    }
                }
                if proposed_direction.is_none() {
                    proposed_direction = Some(direction[0]);
                    if has_neighbor {
                        break;
                    }
                }
            }

            if let Some((dx, dy)) = proposed_direction.filter(|_| has_neighbor) {
                match proposals.entry((x + dx, y + dy)) {
                    Entry::Occupied(_) => {
                        conflicts.insert((x + dx, y + dy));
                        proposals.insert((x, y), (x, y));
                    }
                    Entry::Vacant(e) => {
                        e.insert((x, y));
                    }
                }
            } else {
                proposals.insert((x, y), (x, y));
            }
        }
        for conflict in &conflicts {
            let orig = proposals.remove(conflict).unwrap();
            proposals.insert(orig, orig);
        }

        assert_eq!(proposals.len(), positions.len());
        positions.clear();
        positions.extend(proposals.keys());
        check_pos_start += 1;
        if check_pos_start == CHECK_POSITIONS.len() {
            check_pos_start = 0;
        }
    }
    let mut min_x = i32::MAX;
    let mut min_y = i32::MAX;
    let mut max_x = i32::MIN;
    let mut max_y = i32::MIN;

    positions.iter().for_each(|&(x, y)| {
        min_x = min_x.min(x);
        min_y = min_y.min(y);
        max_x = max_x.max(x + 1);
        max_y = max_y.max(y + 1);
    });

    (max_x - min_x) * (max_y - min_y) - positions.len() as i32
}

pub fn part_2(positions: &HashSet<Pos>) -> u32 {
    let mut check_pos_start = 0;
    let mut proposals = HashMap::with_capacity(positions.len());
    let mut conflicts = HashSet::with_capacity(positions.len());
    let mut positions = positions.clone();

    for i in 1.. {
        proposals.clear();
        conflicts.clear();
        for &(x, y) in &positions {
            let mut has_neighbor = false;
            let mut proposed_direction = None;
            'outer: for direction in CHECK_POSITIONS[check_pos_start..]
                .iter()
                .chain(&CHECK_POSITIONS[..check_pos_start])
            {
                for &(dx, dy) in direction {
                    if positions.contains(&(x + dx, y + dy)) {
                        has_neighbor = true;
                        if proposed_direction.is_some() {
                            break 'outer;
                        } else {
                            continue 'outer;
                        }
                    }
                }
                if proposed_direction.is_none() {
                    proposed_direction = Some(direction[0]);
                    if has_neighbor {
                        break;
                    }
                }
            }

            if let Some((dx, dy)) = proposed_direction.filter(|_| has_neighbor) {
                match proposals.entry((x + dx, y + dy)) {
                    Entry::Occupied(_) => {
                        conflicts.insert((x + dx, y + dy));
                        proposals.insert((x, y), (x, y));
                    }
                    Entry::Vacant(e) => {
                        e.insert((x, y));
                    }
                }
            } else {
                proposals.insert((x, y), (x, y));
            }
        }
        for conflict in &conflicts {
            let orig = proposals.remove(conflict).unwrap();
            proposals.insert(orig, orig);
        }

        if proposals.iter().all(|(&to, &from)| to == from) {
            return i;
        }
        positions.clear();
        positions.extend(proposals.keys());
        check_pos_start += 1;
        if check_pos_start == CHECK_POSITIONS.len() {
            check_pos_start = 0;
        }
    }

    panic!("Expected to stop")
}
