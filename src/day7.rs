pub fn generator(s: &str) -> Vec<u32> {
    let mut results = Vec::with_capacity(1024);
    let mut in_progress = Vec::with_capacity(64);

    // Add a root directory
    in_progress.push(0);

    for line in s.lines() {
        if line == "$ cd /" || line == "$ ls" || line.starts_with("dir ") {
            continue;
        } else if line == "$ cd .." {
            results.push(in_progress.pop().unwrap());
        } else if line.starts_with("$ cd ") {
            in_progress.push(0u32);
        } else {
            let (size, _name) = line.split_once(' ').unwrap();
            let size: u32 = size.parse().unwrap();
            in_progress.iter_mut().for_each(|d| *d += size);
        }
    }
    results.extend_from_slice(&in_progress);
    results.sort_unstable();
    results
}

pub fn part_1(sizes: &[u32]) -> u32 {
    let idx = sizes.partition_point(|&size| size < 100_000);
    sizes[..idx].iter().sum()
}

pub fn part_2(sizes: &[u32]) -> u32 {
    const DISK_SIZE: u32 = 70_000_000;
    const NEEDED_SPACE: u32 = 30_000_000;

    // Last item will be the root directory, since it has the largest size
    let available_space = DISK_SIZE - sizes.last().unwrap();
    let required_to_free = NEEDED_SPACE - available_space;

    // will point to the first item which fails the predicate
    let idx = sizes.partition_point(|&size| size < required_to_free);
    sizes[idx]
}

super::day_test! {demo_1 == 95437}
super::day_test! {demo_2 == 24933642}
super::day_test! {part_1 == 1644735}
super::day_test! {part_2 == 1300850}
