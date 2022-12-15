use std::cmp::Ordering;

#[derive(Clone, Debug)]
pub enum Item {
    Num(u8),
    List(Vec<Item>),
}

impl PartialEq for Item {
    fn eq(&self, other: &Self) -> bool {
        self.cmp(other) == Ordering::Equal
    }
}

impl Eq for Item {}

impl PartialOrd for Item {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Item {
    fn cmp(&self, other: &Self) -> Ordering {
        match (self, other) {
            (Self::Num(lhs), Self::Num(rhs)) => lhs.cmp(rhs),
            (Self::List(lhs), Self::List(rhs)) => {
                for (l, r) in lhs.iter().zip(rhs.iter()) {
                    let cmp = l.cmp(r);
                    if cmp != Ordering::Equal {
                        return cmp;
                    }
                }
                // All items same, just based on length now
                lhs.len().cmp(&rhs.len())
            }
            (list @ Self::List(_), &Self::Num(n)) => list.cmp(&Item::List(vec![Self::Num(n)])),
            (&Self::Num(n), list @ Self::List(_)) => Item::List(vec![Self::Num(n)]).cmp(list),
        }
    }
}

pub fn generator(s: &str) -> Vec<Vec<Item>> {
    let mut result = Vec::with_capacity(512);
    let mut stack: Vec<Vec<Item>> = Vec::with_capacity(32);
    for line in s.lines() {
        if line.is_empty() {
            continue;
        }
        let mut n = None;
        // Ignore the last pop, so we end up with a single list at the top level
        for b in line[..line.len() - 1].bytes() {
            match b {
                b'[' => stack.push(Vec::with_capacity(32)),
                b']' => {
                    if let Some(n) = n.take() {
                        stack.last_mut().unwrap().push(Item::Num(n));
                    }
                    let finished_list = stack.pop().unwrap();
                    stack.last_mut().unwrap().push(Item::List(finished_list));
                }
                b',' => {
                    if let Some(n) = n.take() {
                        stack.last_mut().unwrap().push(Item::Num(n));
                    }
                }
                digit => {
                    debug_assert!((b'0'..=b'9').contains(&digit));
                    let current_n = n.get_or_insert(0);
                    *current_n = *current_n * 10 + (digit - b'0');
                }
            }
        }
        let mut message = stack.pop().unwrap();
        if let Some(n) = n {
            message.push(Item::Num(n))
        }
        result.push(message);
        assert!(stack.is_empty());
    }
    result
}

pub fn part_1(messages: &[Vec<Item>]) -> usize {
    let mut sum = 0;
    for (i, two_messages) in messages.chunks(2).enumerate() {
        let [m1, m2] = two_messages else { panic!("Only 2 items") };
        if m1 < m2 {
            sum += i + 1;
        }
    }
    sum
}

pub fn part_2(messages: &[Vec<Item>]) -> usize {
    let divider_1 = vec![Item::List(vec![Item::Num(2)])];
    let divider_2 = vec![Item::List(vec![Item::Num(6)])];
    let mut messages = messages.to_vec();
    messages.sort_unstable();
    let idx1 = messages.binary_search(&divider_1).unwrap_err() + 1;
    // 1 indexed, plus the position the first divider would have taken
    let idx2 = messages.binary_search(&divider_2).unwrap_err() + 2;

    idx1 * idx2
}

super::day_test! {demo_1 == 13}
super::day_test! {part_1 == 5198}
super::day_test! {demo_2 == 140}
super::day_test! {part_2 == 22344}
