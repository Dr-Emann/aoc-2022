type Range = (u32, u32);
type Pair = (Range, Range);

fn parse_range(s: &str) -> Range {
    let (begin, end) = s.split_once('-').unwrap();
    let begin: u32 = begin.parse().unwrap();
    let end: u32 = end.parse().unwrap();

    (begin, end)
}

pub fn generator(s: &str) -> Vec<Pair> {
    let mut result = Vec::with_capacity(1024);
    s.lines().for_each(|line| {
        let (l, r) = line.split_once(',').unwrap();
        result.push((parse_range(l), parse_range(r)))
    });
    result
}

pub fn part_1(assignments: &[Pair]) -> usize {
    assignments
        .iter()
        .filter(|(l, r)| (l.0 <= r.0 && l.1 >= r.1) || (r.0 <= l.0 && r.1 >= l.1))
        .count()
}

pub fn part_2(assignments: &[Pair]) -> usize {
    assignments
        .iter()
        .filter(|(l, r)| l.0 <= r.1 && l.1 >= r.0)
        .count()
}
