use std::{cmp, mem};

type Worry = u64;
type MonkeyId = u8;

#[derive(Debug, Copy, Clone)]
enum Op {
    Add(Worry),
    Mul(Worry),
    Square,
}

#[derive(Debug, Clone)]
pub struct Monkey {
    worries: Vec<Worry>,
    op: Op,
    divisible_check: Worry,
    true_monkey: MonkeyId,
    false_monkey: MonkeyId,
    items_inspected: u64,
}

pub fn generator(s: &str) -> Vec<Monkey> {
    let mut result = Vec::with_capacity(16);

    let mut lines = s.lines();

    while let Some(_header) = lines.next() {
        let worries: Vec<Worry> = lines
            .next()
            .unwrap()
            .strip_prefix("  Starting items: ")
            .unwrap()
            .split(", ")
            .map(|s| s.parse().unwrap())
            .collect();
        let op_line = lines
            .next()
            .unwrap()
            .strip_prefix("  Operation: new = old ")
            .unwrap();
        let op_value = &op_line[2..];
        let op = match op_line.as_bytes()[0] {
            b'*' if op_value == "old" => Op::Square,
            b'*' => Op::Mul(op_value.parse().unwrap()),
            b'+' => Op::Add(op_value.parse().unwrap()),
            _ => panic!("Unimplemented operation"),
        };

        let div: Worry = lines
            .next()
            .unwrap()
            .strip_prefix("  Test: divisible by ")
            .unwrap()
            .parse()
            .unwrap();

        let true_idx: MonkeyId = lines
            .next()
            .unwrap()
            .strip_prefix("    If true: throw to monkey ")
            .unwrap()
            .parse()
            .unwrap();
        let false_idx: MonkeyId = lines
            .next()
            .unwrap()
            .strip_prefix("    If false: throw to monkey ")
            .unwrap()
            .parse()
            .unwrap();

        // empty line (if any)
        let _ = lines.next();

        result.push(Monkey {
            worries,
            op,
            divisible_check: div,
            true_monkey: true_idx,
            false_monkey: false_idx,
            items_inspected: 0,
        });
    }

    result
}

fn gcd(mut a: Worry, mut b: Worry) -> Worry {
    while b != 0 {
        (a, b) = (b, a % b);
    }
    a
}

fn lcm(a: Worry, b: Worry) -> Worry {
    a / gcd(a, b) * b
}

fn get_monkey_with_destinations(monkeys: &mut [Monkey], i: usize) -> [&mut Monkey; 3] {
    let start = monkeys.as_ptr();
    let monkey = &monkeys[i];
    let true_idx = usize::from(monkey.true_monkey);
    let false_idx = usize::from(monkey.false_monkey);

    let mut idxs = [i, true_idx, false_idx];
    idxs.sort_unstable();

    let mut monkeys = monkeys.iter_mut();
    let mut selected = [
        monkeys.nth(idxs[0]).unwrap(),
        monkeys.nth(idxs[1] - idxs[0] - 1).unwrap(),
        monkeys.nth(idxs[2] - idxs[1] - 1).unwrap(),
    ];

    if idxs[0] != i {
        if idxs[1] == i {
            idxs.swap(0, 1);
            selected.swap(0, 1);
        } else {
            idxs.swap(0, 2);
            selected.swap(0, 2);
        }
    }
    if idxs[1] != true_idx {
        selected.swap(1, 2);
    }

    debug_assert_eq!(
        ((selected[0] as *mut _ as usize) - start as usize) / mem::size_of::<Monkey>(),
        i
    );
    debug_assert_eq!(
        ((selected[1] as *mut _ as usize) - start as usize) / mem::size_of::<Monkey>(),
        true_idx
    );
    debug_assert_eq!(
        ((selected[2] as *mut _ as usize) - start as usize) / mem::size_of::<Monkey>(),
        false_idx
    );

    selected
}

fn iterate_monkeys(monkeys: &mut [Monkey], iterations: u32, reduce_worry: bool) -> u64 {
    let modulus: Worry = monkeys.iter().fold(1, |acc, m| lcm(acc, m.divisible_check));

    for _ in 0..iterations {
        for i in 0..monkeys.len() {
            let monkey = &mut monkeys[i];
            for item in &mut monkey.worries {
                match monkey.op {
                    Op::Add(x) => *item += x,
                    Op::Mul(x) => *item *= x,
                    Op::Square => *item *= *item,
                }
                if reduce_worry {
                    *item /= 3;
                }
                *item %= modulus;
            }
            monkey.items_inspected += u64::try_from(monkey.worries.len()).unwrap();

            let [monkey, true_monkey, false_monkey] = get_monkey_with_destinations(monkeys, i);
            let (true_items, false_items) =
                partition::partition(&mut monkey.worries, |&w| w % monkey.divisible_check == 0);
            true_monkey.worries.extend_from_slice(true_items);
            false_monkey.worries.extend_from_slice(false_items);
            monkey.worries.clear();
        }
    }

    let (top_monkeys, _, _) =
        monkeys.select_nth_unstable_by_key(2, |m| cmp::Reverse(m.items_inspected));
    top_monkeys.iter().map(|m| m.items_inspected).product()
}

pub fn part_1(monkeys: &[Monkey]) -> u64 {
    let mut monkeys = monkeys.to_vec();

    iterate_monkeys(&mut monkeys, 20, true)
}

pub fn part_2(monkeys: &[Monkey]) -> u64 {
    let mut monkeys = monkeys.to_vec();

    iterate_monkeys(&mut monkeys, 10_000, false)
}

super::day_test! {demo_1 == 10605}
super::day_test! {part_1 == 61005}
super::day_test! {demo_2 == 2713310158}
super::day_test! {part_2 == 20567144694}
