use std::collections::HashMap;
use std::ops::ControlFlow;

#[derive(Debug, Default)]
pub struct Directory<'a> {
    sub_dirs: HashMap<&'a str, Box<Directory<'a>>>,
    total_size: u32,
}

fn populate_dir<'a>(dir: &mut Directory<'a>, lines: &mut impl Iterator<Item = &'a str>) {
    while let Some(line) = lines.next() {
        if line == "$ cd .." {
            return;
        } else if line == "$ ls" {
            // Do nothing
        } else if let Some(dst) = line.strip_prefix("$ cd ") {
            let subdir = dir.sub_dirs.get_mut(dst).unwrap();
            let prev_subsize = subdir.total_size;
            populate_dir(subdir, lines);
            let new_subsize = subdir.total_size;
            dir.total_size += new_subsize - prev_subsize;
        } else if let Some(dir_name) = line.strip_prefix("dir ") {
            dir.sub_dirs
                .insert(dir_name, Box::new(Directory::default()));
        } else {
            let (size, _name) = line.split_once(' ').unwrap();
            let size: u32 = size.parse().unwrap();
            dir.total_size += size;
        }
    }
}

pub fn generator(s: &str) -> Directory {
    let mut root = Directory::default();

    let mut lines = s.lines();
    assert_eq!(lines.next().unwrap(), "$ cd /");

    populate_dir(&mut root, &mut lines);

    root
}

pub fn visit_smaller_dirs(d: &Directory, visit: &mut impl FnMut(&Directory) -> ControlFlow<()>) {
    if visit(d).is_break() {
        return;
    }
    for sub_dir in d.sub_dirs.values() {
        visit_smaller_dirs(sub_dir, visit);
    }
}

pub fn part_1(d: &Directory) -> u32 {
    let mut total_size = 0;
    visit_smaller_dirs(d, &mut |d| {
        if d.total_size <= 100_000 {
            total_size += d.total_size
        }
        ControlFlow::Continue(())
    });
    total_size
}

pub fn part_2(d: &Directory) -> u32 {
    const DISK_SIZE: u32 = 70_000_000;
    const NEEDED_SPACE: u32 = 30_000_000;

    let available_space = DISK_SIZE - d.total_size;
    let required_to_free = NEEDED_SPACE - available_space;
    let mut best_space = u32::MAX;

    visit_smaller_dirs(d, &mut |d| {
        if d.total_size < required_to_free {
            return ControlFlow::Break(());
        }

        best_space = best_space.min(d.total_size);

        ControlFlow::Continue(())
    });

    best_space
}

super::day_test! {demo_1 == 95437}
super::day_test! {demo_2 == 24933642}
super::day_test! {part_1 == 1644735}
super::day_test! {part_2 == 1300850}
