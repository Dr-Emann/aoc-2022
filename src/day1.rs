use std::mem;

#[derive(Debug)]
pub struct Groups(Vec<u32>);

pub fn generator(s: &str) -> Groups {
    let mut groups = Vec::with_capacity(100);
    let mut current_group = 0u32;
    for line in s.lines() {
        if line.is_empty() {
            if current_group != 0 {
                groups.push(mem::replace(&mut current_group, 0));
            }
            continue;
        }
        let n: u32 = line.parse().unwrap();
        current_group += n;
    }

    if current_group != 0 {
        groups.push(current_group)
    }

    Groups(groups)
}

pub fn part_1(groups: &Groups) -> u32 {
    groups.0.iter().copied().max().unwrap()
}

pub fn part_2(groups: &Groups) -> u32 {
    let mut groups = groups.0.clone();
    groups.select_nth_unstable_by_key(2, |i| std::cmp::Reverse(*i));
    groups[..3].iter().sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    const BODY: &str = include_str!("../input/2022/day1.txt");

    #[test]
    fn p1() {
        let input = generator(BODY);
        assert_eq!(part_1(&input), 74394);
    }

    #[test]
    fn p2() {
        let input = generator(BODY);
        assert_eq!(part_2(&input), 212836);
    }
}
