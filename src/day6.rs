pub fn generator(s: &str) -> &str {
    s.trim_end()
}

pub fn part_1(s: &str) -> usize {
    for (i, window) in s.as_bytes().windows(4).enumerate() {
        if !window[1..].contains(&window[0])
            && !window[2..].contains(&window[1])
            && window[2] != window[3]
        {
            return i + 4;
        }
    }
    unreachable!()
}

pub fn part_2(s: &str) -> usize {
    'outer: for (i, window) in s.as_bytes().windows(14).enumerate() {
        for j in 0..13 {
            if window[j + 1..].contains(&window[j]) {
                continue 'outer;
            }
        }
        return i + 14;
    }
    unreachable!()
}
