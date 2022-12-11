use std::cmp;

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
            let (true_items, false_items): (Vec<_>, Vec<_>) = monkey
                .worries
                .drain(..)
                .partition(|&item| item % monkey.divisible_check == 0);

            let true_idx = monkey.true_monkey;
            let false_idx = monkey.false_monkey;
            monkeys[usize::from(true_idx)]
                .worries
                .extend_from_slice(&true_items);
            monkeys[usize::from(false_idx)]
                .worries
                .extend_from_slice(&false_items);
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
super::day_test! {part_2 == 2713310158}
