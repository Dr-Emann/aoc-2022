#[derive(Debug, Copy, Clone)]
pub struct Rucksack<'a>(&'a [u8]);

impl<'a> Rucksack<'a> {
    fn split_set(self) -> (u64, u64) {
        let (left, right) = self.0.split_at(self.0.len() / 2);
        (set(left), set(right))
    }

    fn set(self) -> u64 {
        set(self.0)
    }
}

fn set(items: &[u8]) -> u64 {
    let mut result = 0;

    for &item in items {
        let shift = match item {
            b'a'..=b'z' => item - b'a',
            _ => {
                debug_assert!((b'A'..=b'Z').contains(&item));
                item - b'A' + 26
            }
        };
        result |= 1 << shift;
    }

    result
}

fn priority_from_set(set: u64) -> u32 {
    debug_assert_eq!(set.count_ones(), 1);

    let bit_set = set.trailing_zeros();
    bit_set + 1
}

pub fn generator(s: &str) -> Vec<Rucksack> {
    let mut result = Vec::with_capacity(1024);
    s.lines()
        .for_each(|line| result.push(Rucksack(line.as_bytes())));
    result
}

pub fn part_1(bags: &[Rucksack]) -> u32 {
    let mut total_score = 0;
    for (left, right) in bags.iter().copied().map(Rucksack::split_set) {
        let intersection = left & right;

        total_score += priority_from_set(intersection);
    }
    total_score
}

pub fn part_2(bags: &[Rucksack]) -> u32 {
    let mut total_score = 0;
    for chunk in bags.chunks_exact(3) {
        let intersection = chunk.iter().fold(!0, |i, sack| i & sack.set());
        total_score += priority_from_set(intersection);
    }
    total_score
}

super::day_test! {demo_1 == 157}
super::day_test! {demo_2 == 70}
super::day_test! {part_1 == 7908}
super::day_test! {part_2 == 2838}
