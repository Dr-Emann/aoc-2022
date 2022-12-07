use std::str::FromStr;

type Range = (u32, u32);
type Pair = (Range, Range);

pub fn generator(s: &str) -> Vec<Pair> {
    let mut result = Vec::with_capacity(1024);
    s.lines().for_each(|line| {
        let mut items = line.split([',', '-']).map(|s| u32::from_str(s).unwrap());

        result.push((
            (items.next().unwrap(), items.next().unwrap()),
            (items.next().unwrap(), items.next().unwrap()),
        ))
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

super::day_test! {demo_1 == 2}
super::day_test! {demo_2 == 4}
super::day_test! {part_1 == 441}
super::day_test! {part_2 == 861}
