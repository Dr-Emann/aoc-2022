use aoc_2022::DAYS;
use criterion::{black_box, criterion_group, criterion_main, Criterion};
use std::fs;

pub fn full_bench(c: &mut Criterion) {
    let mut days_inputs = Vec::with_capacity(25);
    for (i, f) in DAYS.iter().copied().enumerate() {
        let day = i + 1;
        if let Some(f) = f {
            let input = fs::read_to_string(format!("input/2022/day{day}.txt")).unwrap();
            days_inputs.push((day, f, input));
        }
    }
    c.bench_function("all_days", |b| {
        b.iter(|| {
            for (_day, f, input) in &days_inputs {
                f(black_box(input), false);
            }
        })
    });
    let mut group = c.benchmark_group("days");
    for (day, f, input) in &days_inputs {
        group.bench_function(&day.to_string(), |b| {
            b.iter(|| {
                f(black_box(input), false);
            })
        });
    }
}

criterion_group!(benches, full_bench);
criterion_main!(benches);
