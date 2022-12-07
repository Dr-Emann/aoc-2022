pub fn generator(s: &str) -> &str {
    s.trim_end()
}

pub fn part_1(s: &str) -> usize {
    find_non_duplicate_window(s, 4)
}

pub fn part_2(s: &str) -> usize {
    find_non_duplicate_window(s, 14)
}

// Find the first `window_size` consecutive bytes containing no duplicates
fn find_non_duplicate_window(s: &str, window_size: usize) -> usize {
    let s = s.as_bytes();
    let mut idx = 0;
    'outer: loop {
        let window = &s[idx..][..window_size];

        // Find a pair of duplicates, starting at the end of the window
        // once we find a pair of duplicates, we can skip until the window is past
        // the earlier of the two duplicates.
        // Start looking at the end of the window, because we want to be able to skip
        // more bytes: the best case is we find the last two bytes are duplicates, and we
        // can skip almost the whole window.
        for (i, b) in window[..window.len() - 1].iter().enumerate().rev() {
            if window[i + 1..].contains(b) {
                idx += i + 1;
                continue 'outer;
            }
        }
        return idx + window_size;
    }
}

super::day_test! {demo_1 == 7}
super::day_test! {demo_2 == 19}
super::day_test! {part_1 == 1287}
super::day_test! {part_2 == 3716}
