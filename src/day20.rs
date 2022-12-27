pub fn generator(s: &str) -> Vec<i64> {
    let mut result = Vec::with_capacity(1024);
    result.extend(s.lines().map(|l| l.parse::<i64>().unwrap()));
    result
}

pub fn part_1(items: &[i64]) -> i64 {
    let mixed = mix(items, 1);
    let start_idx = mixed.iter().position(|&item| item == 0).unwrap();

    let mut sum = 0;
    for pos in POSITIONS {
        sum += mixed[(pos + start_idx) % mixed.len()];
    }
    sum
}

pub fn part_2(items: &[i64]) -> i64 {
    const DECRYPTION_KEY: i64 = 811_589_153;
    let items: Vec<i64> = items.iter().map(|&item| item * DECRYPTION_KEY).collect();

    let mixed = mix(&items, 10);
    let start_idx = mixed.iter().position(|&item| item == 0).unwrap();

    let mut sum = 0;
    for pos in POSITIONS {
        sum += mixed[(pos + start_idx) % mixed.len()];
    }
    sum
}

const POSITIONS: [usize; 3] = [1000, 2000, 3000];

fn mix(items: &[i64], count: u32) -> Vec<i64> {
    let len_less_one = i64::try_from(items.len()).unwrap() - 1;
    let mut indexes: Vec<u16> = (0..u16::try_from(items.len()).unwrap()).collect();
    for _ in 0..count {
        for (i, &item) in items.iter().enumerate() {
            let i = i as u16;
            let actual_idx = indexes.iter().position(|&idx| idx == i).unwrap();
            debug_assert!(!indexes[actual_idx + 1..].contains(&i));
            let new_idx =
                (i64::try_from(actual_idx).unwrap() + item).rem_euclid(len_less_one) as usize;

            indexes.remove(actual_idx);
            indexes.insert(new_idx, i);
        }
    }
    indexes
        .into_iter()
        .map(|idx| items[usize::from(idx)])
        .collect()
}

super::day_test! {demo_1 == 3}
super::day_test! {demo_2 == 1623178306}
super::day_test! {part_1 == 7004}
super::day_test! {part_2 == 17200008919529}
