use std::fmt;
use std::fmt::Write;

#[derive(Clone)]
struct CrateStacks {
    items: Vec<Vec<u8>>,
}

impl CrateStacks {
    fn apply<const REVERSE: bool>(&mut self, op: &Operation) {
        let Operation {
            count,
            source,
            dest,
        } = *op;

        // Convert to 0 based index
        let source = source - 1;
        let dest = dest - 1;

        debug_assert_ne!(source, dest);

        let (source_stack, dest_stack) = {
            let (left, right) = self.items.split_at_mut(source.max(dest));
            if source < dest {
                (&mut left[source], &mut right[0])
            } else {
                (&mut right[0], &mut left[dest])
            }
        };

        let new_len = source_stack.len() - count;
        if REVERSE {
            source_stack[new_len..].reverse();
        }
        dest_stack.extend_from_slice(&source_stack[new_len..]);
        source_stack.truncate(new_len);
    }
}

#[derive(Debug, Copy, Clone)]
struct Operation {
    count: usize,
    source: usize,
    dest: usize,
}

#[derive(Debug, Clone)]
pub struct Input {
    stacks: CrateStacks,
    operations: Vec<Operation>,
}

pub fn generator(s: &str) -> Input {
    let (stacks, operations) = s.split_once("\n\n").unwrap();
    let stacks = parse_stacks(stacks);
    let operations = parse_operations(operations);

    Input { stacks, operations }
}

pub fn part_1(input: &Input) -> String {
    let mut stacks = input.stacks.clone();
    for op in &input.operations {
        stacks.apply::<true>(op);
    }

    let mut word = String::with_capacity(stacks.items.len());
    for column in &stacks.items {
        word.push(column.last().copied().unwrap() as char)
    }

    word
}

pub fn part_2(input: &Input) -> String {
    let mut stacks = input.stacks.clone();
    for op in &input.operations {
        stacks.apply::<false>(op);
    }

    let mut word = String::with_capacity(stacks.items.len());
    for column in &stacks.items {
        word.push(column.last().copied().unwrap() as char)
    }

    word
}

fn parse_stacks(s: &str) -> CrateStacks {
    let mut width = 0;
    let mut lines = s.lines();
    // Drop the last line
    lines.next_back();

    for line in lines.clone() {
        width = width.max((line.len() + 1) / 4);
    }

    let mut columns = Vec::with_capacity(width);
    for _ in 0..width {
        columns.push(Vec::with_capacity(1024));
    }

    for line in lines.rev() {
        for (chunk, column) in line.as_bytes().chunks(4).zip(&mut columns) {
            if chunk[1] != b' ' {
                column.push(chunk[1])
            }
        }
    }
    CrateStacks { items: columns }
}

fn parse_operations(s: &str) -> Vec<Operation> {
    let mut result = Vec::with_capacity(1024);
    for line in s.lines() {
        let mut words = line.split(' ');

        let r#move = words.next().unwrap_or_default();
        let count = words.next().unwrap_or_default();
        let from = words.next().unwrap_or_default();
        let source = words.next().unwrap_or_default();
        let to = words.next().unwrap_or_default();
        let dest = words.next().unwrap_or_default();

        debug_assert!(words.next().is_none());
        debug_assert_eq!(r#move, "move");
        debug_assert_eq!(from, "from");
        debug_assert_eq!(to, "to");

        result.push(Operation {
            count: count.parse().unwrap(),
            source: source.parse().unwrap(),
            dest: dest.parse().unwrap(),
        });
    }

    result
}

impl fmt::Debug for CrateStacks {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_char('\n')?;
        let max_height = self.items.iter().map(|col| col.len()).max().unwrap();
        for i in (0..max_height).rev() {
            for col in &self.items {
                let ch = col.get(i).map(|&b| b as char).unwrap_or(' ');
                f.write_char(ch)?;
                f.write_char(' ')?;
            }
            f.write_char('\n')?;
        }
        Ok(())
    }
}

super::day_test! {demo_1 == "CMZ"}
super::day_test! {demo_2 == "MCD"}
super::day_test! {part_1 == "VRWBSFZWM"}
super::day_test! {part_2 == "RBTWJWMCF"}
