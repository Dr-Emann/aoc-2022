use clap::Parser;
use linkme::distributed_slice;
use std::fmt::Display;
use std::path::PathBuf;
use std::time::{Duration, Instant};
use std::{fs, panic};

// TODO: This doesn't need linkme or anything
day!(day1);
day!(day2);
day!(day3);
day!(day4);
day!(day5);
day!(day6);

#[derive(clap::Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Day to run
    ///
    /// If not passed, all days are run in order
    day: Option<u32>,

    /// Path to load input from (defaults to path in input/2022 based on day name)
    input: Option<PathBuf>,

    /// Try to load demo input
    #[arg(short, long)]
    demo: bool,

    #[arg(short, long, conflicts_with = "day")]
    latest: bool,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();
    let single_day = args.day.or_else(|| {
        if args.latest {
            Some(latest_day())
        } else {
            None
        }
    });
    if let Some(day) = single_day {
        println!("Day {day}");
        let input_path = args.input.unwrap_or_else(|| input_for_day(day, args.demo));
        let input = fs::read_to_string(input_path)?;

        let runner = find_day(day).ok_or_else(|| format!("Day {day} not implemented"))?;

        runner(&input);
        return Ok(());
    }

    let mut days = DAYS.to_vec();
    days.sort_by_key(|(d, _f)| *d);

    let overall_start = Instant::now();
    for (i, (day, runner)) in days.into_iter().enumerate() {
        if i != 0 {
            println!();
        }
        println!("Day {}", day);
        let input_path = input_for_day(day, args.demo);
        let input = fs::read_to_string(input_path)?;

        runner(&input);
    }
    let total_time = overall_start.elapsed();
    println!();
    println!("Total time: {:.2?}", total_time);
    Ok(())
}

fn latest_day() -> u32 {
    DAYS.iter().map(|(d, _)| *d).max().unwrap()
}

fn find_day(day: u32) -> Option<fn(&str)> {
    DAYS.iter().find(|(d, _)| day == *d).map(|(_, f)| *f)
}

fn input_for_day(day: u32, demo: bool) -> PathBuf {
    let prefix = if demo { "demo" } else { "day" };
    PathBuf::from(format!("input/2022/{prefix}{day}.txt"))
}

#[test]
fn verify_args() {
    use clap::CommandFactory;
    Args::command().debug_assert()
}

#[allow(dead_code)]
fn unimplemented_part<I>(_input: &I) -> &'static str {
    "Unimplemented"
}

macro_rules! day {
    ($mod_name:ident) => {
        $crate::day! {$mod_name, $mod_name::generator, $mod_name::part_1, $mod_name::part_2 }
    };
    ($mod_name:ident, $gen:expr, $part1:expr, $part2:expr) => {
        mod $mod_name;

        // Hide names from outside
        const _: () = {
            #[allow(unused_imports)]
            use $mod_name::*;

            fn run_day(s: &str) {
                let (gen_elapsed, input) = $crate::time(|| $gen(s));
                let input = match input {
                    Ok(i) => i,
                    Err(e) => {
                        println!("Generator error: {e}");
                        return;
                    }
                };
                let (p1_elapsed, p1_result) = $crate::time(|| $part1(&input));
                let (p2_elapsed, p2_result) = $crate::time(|| $part2(&input));

                let p1_result = $crate::stringify_res(p1_result);
                let p2_result = $crate::stringify_res(p2_result);

                println!("Gen    ({:.2?})", gen_elapsed);
                println!("Part 1 ({:.2?}) {p1_result}", p1_elapsed);
                println!("Part 2 ({:.2?}) {p2_result}", p2_elapsed);
                println!("Total  ({:.2?})", gen_elapsed + p1_elapsed + p2_elapsed);
            }

            #[linkme::distributed_slice($crate::DAYS)]
            static DAY: (u32, fn(&str)) =
                ($crate::extract_day_number(stringify!($mod_name)), run_day);
        };
    };
}

const fn extract_day_number(s: &str) -> u32 {
    let mut day_number = 0;
    let s = s.as_bytes();
    assert!(s[0] == b'd');
    assert!(s[1] == b'a');
    assert!(s[2] == b'y');

    let mut i = 3;
    while i < s.len() {
        let digit = s[i];
        assert!(digit.is_ascii_digit());
        let val = (digit - b'0') as u32;
        day_number *= 10;
        day_number += val;
        i += 1;
    }

    day_number
}

pub(crate) use day;

#[distributed_slice]
static DAYS: [(u32, fn(&str))] = [..];

fn time<T, F: FnOnce() -> T + panic::UnwindSafe>(f: F) -> (Duration, Result<T, String>) {
    let start = Instant::now();
    let result = run_catch_panic(f);
    let duration = start.elapsed();
    (duration, result)
}

fn run_catch_panic<T, F>(f: F) -> Result<T, String>
where
    F: FnOnce() -> T,
    F: panic::UnwindSafe,
{
    panic::catch_unwind(f).map_err(|e| {
        let s: &str = e
            .downcast_ref::<&str>()
            .copied()
            .or_else(|| e.downcast_ref::<String>().map(|s| &**s))
            .unwrap_or("Unknown");
        format!("panic'd: {}", s)
    })
}

fn stringify_res<T: Display, E: Display>(r: Result<T, E>) -> String {
    match r {
        Ok(t) => t.to_string(),
        Err(e) => e.to_string(),
    }
}

#[test]
fn unique_days() {
    let mut days: Vec<u32> = DAYS.iter().map(|(d, _)| *d).collect();
    days.sort();
    days.dedup();
    assert_eq!(days.len(), DAYS.len());
}
