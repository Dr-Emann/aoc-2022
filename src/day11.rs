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
struct Monkey {
    worries: Vec<Worry>,
    op: Op,
    divisible_check: Worry,
    true_monkey: MonkeyId,
    false_monkey: MonkeyId,
    items_inspected: u64,
}

impl Monkey {
    fn inspect_items<const REDUCE_WORRY: bool>(&mut self, lcm_modulus: Worry) {
        for item in &mut self.worries {
            match self.op {
                Op::Add(x) => *item += x,
                Op::Mul(x) => *item *= x,
                Op::Square => *item *= *item,
            }
            if REDUCE_WORRY {
                *item /= 3;
            }
            *item %= lcm_modulus;
        }
        self.items_inspected += u64::try_from(self.worries.len()).unwrap();
    }
}

#[derive(Debug, Clone)]
pub struct Monkeys {
    monkeys: Box<[Monkey]>,
    lcm_modulus: Worry,
}

impl Monkeys {
    fn new(monkeys: Box<[Monkey]>) -> Self {
        let mut lcm_modulus = 1;
        let monkeys_len = monkeys.len();
        for (i, monkey) in monkeys.iter().enumerate() {
            lcm_modulus = lcm(lcm_modulus, monkey.divisible_check);

            let true_monkey = usize::from(monkey.true_monkey);
            let false_monkey = usize::from(monkey.false_monkey);

            // Ensure all indexes are in range, and non-overlapping with each other or `i`
            // this assurance is used in the unsafe block when getting all 3 monkeys via &mut at the same time
            assert!(true_monkey < monkeys_len);
            assert!(false_monkey < monkeys_len);

            assert_ne!(true_monkey, i);
            assert_ne!(false_monkey, i);
            assert_ne!(true_monkey, false_monkey);
        }

        Self {
            monkeys,
            lcm_modulus,
        }
    }

    fn step<const REDUCE_WORRY: bool>(&mut self) {
        let lcm_modulus = self.lcm_modulus;
        for i in 0..self.monkeys.len() {
            let (monkey, (true_dest, false_dest)) = self.get_with_destinations(i);
            monkey.inspect_items::<REDUCE_WORRY>(lcm_modulus);

            let (true_items, false_items) =
                partition(&mut monkey.worries, |&w| w % monkey.divisible_check == 0);
            true_dest.worries.extend_from_slice(true_items);
            false_dest.worries.extend_from_slice(false_items);
            monkey.worries.clear();
        }
    }

    // Consume self, since this reorders the monkeys
    fn monkey_business(mut self) -> u64 {
        let (top_monkeys, _, _) = self
            .monkeys
            .select_nth_unstable_by_key(2, |m| cmp::Reverse(m.items_inspected));
        top_monkeys[1].items_inspected * top_monkeys[0].items_inspected
    }

    // Return (monkey, (true_dest, false_dest))
    fn get_with_destinations(&mut self, i: usize) -> (&mut Monkey, (&mut Monkey, &mut Monkey)) {
        let (true_idx, false_idx) = {
            let monkey = &self.monkeys[i];
            (
                usize::from(monkey.true_monkey),
                usize::from(monkey.false_monkey),
            )
        };

        debug_assert!(i < self.monkeys.len());
        debug_assert!(true_idx < self.monkeys.len());
        debug_assert!(false_idx < self.monkeys.len());

        debug_assert_ne!(i, true_idx);
        debug_assert_ne!(i, false_idx);
        debug_assert_ne!(true_idx, false_idx);

        let start: *mut Monkey = self.monkeys.as_mut_ptr();
        // SAFETY: All indexes are in bounds, and non-overlapping, as checked in the `new` func,
        //         and lifetimes will borrow self mutably.
        unsafe {
            (
                &mut *start.add(i),
                (&mut *start.add(true_idx), &mut *start.add(false_idx)),
            )
        }
    }
}

pub fn generator(s: &str) -> Monkeys {
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

    Monkeys::new(result.into_boxed_slice())
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

pub fn part_1(monkeys: &Monkeys) -> u64 {
    let mut monkeys = monkeys.clone();

    for _ in 0..20 {
        monkeys.step::<true>();
    }

    monkeys.monkey_business()
}

pub fn part_2(monkeys: &Monkeys) -> u64 {
    let mut monkeys = monkeys.clone();

    for _ in 0..10_000 {
        monkeys.step::<false>();
    }

    monkeys.monkey_business()
}

pub fn partition<T, P>(data: &mut [T], predicate: P) -> (&mut [T], &mut [T])
where
    P: FnMut(&T) -> bool,
{
    let idx = partition_index(data.iter_mut(), predicate);
    data.split_at_mut(idx)
}

// Copied from Iterator::partition_in_place in std
fn partition_index<'a, T, It, P>(mut iter: It, mut predicate: P) -> usize
where
    T: 'a,
    It: DoubleEndedIterator<Item = &'a mut T>,
    P: FnMut(&T) -> bool,
{
    // These closure "factory" functions exist to avoid genericity in `Self`.
    #[inline]
    fn is_false<'a, T>(
        predicate: &'a mut impl FnMut(&T) -> bool,
        true_count: &'a mut usize,
    ) -> impl FnMut(&&mut T) -> bool + 'a {
        move |x| {
            let p = predicate(&**x);
            *true_count += p as usize;
            !p
        }
    }

    #[inline]
    fn is_true<T>(predicate: &mut impl FnMut(&T) -> bool) -> impl FnMut(&&mut T) -> bool + '_ {
        move |x| predicate(&**x)
    }

    let predicate = &mut predicate;
    // Repeatedly find the first `false` and swap it with the last `true`.
    let mut true_count = 0;
    while let Some(head) = iter.find(is_false(predicate, &mut true_count)) {
        if let Some(tail) = iter.rfind(is_true(predicate)) {
            mem::swap(head, tail);
            true_count += 1;
        } else {
            break;
        }
    }
    true_count
}

super::day_test! {demo_1 == 10605}
super::day_test! {part_1 == 61005}
super::day_test! {demo_2 == 2713310158}
super::day_test! {part_2 == 20567144694}
