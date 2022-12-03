#[derive(Debug, Copy, Clone)]
pub struct Rucksack<'a>(&'a [u8]);

impl<'a> Rucksack<'a> {
    fn split(self) -> (&'a [u8], &'a [u8]) {
        self.0.split_at(self.0.len() / 2)
    }
}

pub fn generator(s: &str) -> Vec<Rucksack> {
    s.lines().map(|line| Rucksack(line.as_bytes())).collect()
}

pub fn part_1(bags: &Vec<Rucksack>) -> u32 {
    let mut total_score = 0;
    for (left, right) in bags.iter().copied().map(Rucksack::split) {
        let common_item = find_common_item(left, right);
        total_score += priority(common_item);
    }
    total_score
}

pub fn part_2(bags: &Vec<Rucksack>) -> u32 {
    let mut total_score = 0;
    for chunk in bags.chunks_exact(3) {
        let &[left, middle, right] = chunk else { unreachable!()};
        let item = find_common_item_3(left.0, middle.0, right.0);
        total_score += priority(item);
    }
    total_score
}

fn find_common_item(left: &[u8], right: &[u8]) -> u8 {
    // These lists are small.. faster to just check each item
    for b in left {
        if right.contains(b) {
            return *b;
        }
    }
    panic!("Should have a common element")
}

fn find_common_item_3(left: &[u8], middle: &[u8], right: &[u8]) -> u8 {
    // These lists are small.. still seems faster to just check each item
    for b in left {
        if middle.contains(b) && right.contains(b) {
            return *b;
        }
    }
    panic!("Should have a common element")
}

fn priority(b: u8) -> u32 {
    let zero_based = match b {
        b'a'..=b'z' => b - b'a',
        b'A'..=b'Z' => (b - b'A') + 26,
        _ => panic!(),
    };
    1 + u32::from(zero_based)
}
