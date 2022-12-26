use bitvec::prelude::*;
use std::str::FromStr;

type Pos = [usize; 3];

#[derive(Debug)]
pub struct Field {
    x_len: usize,
    y_len: usize,
    bits: BitVec,
}

impl Field {
    pub fn z_len(&self) -> usize {
        self.bits.len() / (self.x_len * self.y_len)
    }

    fn idx(&self, [x, y, z]: Pos) -> usize {
        debug_assert!(x < self.x_len);
        debug_assert!(y < self.y_len);
        z * self.x_len * self.y_len + y * self.x_len + x
    }

    fn pos(&self, idx: usize) -> Pos {
        let x = idx % self.x_len;
        let y = idx / self.x_len % self.y_len;
        let z = idx / (self.x_len * self.y_len);
        [x, y, z]
    }

    fn get(&self, pos: Pos) -> bool {
        self.bits[self.idx(pos)]
    }
}

pub fn generator(s: &str) -> Field {
    let mut positions = Vec::with_capacity(1024);
    let mut max = [0; 3];
    for line in s.lines() {
        let mut items = line.split(',').map(|n| usize::from_str(n).unwrap());
        let x = items.next().unwrap();
        let y = items.next().unwrap();
        let z = items.next().unwrap();

        // Shift everything, so we have some free space at the beginning
        let point = [x + 1, y + 1, z + 1];

        max.iter_mut()
            .zip(point)
            // Include an extra space on the right side as well
            .for_each(|(m, new)| *m = (*m).max(new + 2));

        positions.push(point);
    }
    let bits = bitvec![0; max[0] * max[1] * max[2]];
    let mut field = Field {
        x_len: max[0],
        y_len: max[1],
        bits,
    };
    positions.iter().for_each(|&pos| {
        let idx = field.idx(pos);
        field.bits.set(idx, true);
    });
    field
}

const ALL_DIRECTIONS: [[isize; 3]; 6] = [
    [-1, 0, 0],
    [1, 0, 0],
    [0, -1, 0],
    [0, 1, 0],
    [0, 0, -1],
    [0, 0, 1],
];

pub fn part_1(field: &Field) -> u32 {
    let mut surface_area = 0;
    for idx in field.bits.iter_ones() {
        let [x, y, z] = field.pos(idx);
        for [dx, dy, dz] in ALL_DIRECTIONS {
            let new_x = x.add_signed(dx);
            let new_y = y.add_signed(dy);
            let new_z = z.add_signed(dz);

            if !field.get([new_x, new_y, new_z]) {
                surface_area += 1;
            }
        }
    }
    surface_area
}

pub fn part_2(field: &Field) -> u32 {
    let mut queue: Vec<Pos> = Vec::with_capacity(1024);
    queue.push([0, 0, 0]);
    let mut visited = bitvec![0; field.bits.len()];

    let mut surface_area = 0;

    while let Some([x, y, z]) = queue.pop() {
        for [dx, dy, dz] in ALL_DIRECTIONS {
            let Some(new_x) = x.checked_add_signed(dx).filter(|&i| i < field.x_len) else { continue };
            let Some(new_y) = y.checked_add_signed(dy).filter(|&i| i < field.y_len) else { continue };
            let Some(new_z) = z.checked_add_signed(dz).filter(|&i| i < field.z_len()) else { continue };

            let new_pos = [new_x, new_y, new_z];
            let idx = field.idx(new_pos);
            if field.get(new_pos) {
                surface_area += 1;
            } else if !visited[idx] {
                visited.set(idx, true);
                queue.push(new_pos);
            }
        }
    }

    surface_area
}

trait UnsignedSigned {
    type Signed;
    fn add_signed(self, other: Self::Signed) -> Self;
}

impl UnsignedSigned for usize {
    type Signed = isize;

    fn add_signed(self, other: Self::Signed) -> Self {
        if cfg!(debug_assertions) {
            self.checked_add_signed(other).unwrap()
        } else {
            self.wrapping_add_signed(other)
        }
    }
}

super::day_test! {demo_1 == 64}
