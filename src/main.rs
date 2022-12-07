use clap::Parser;
use std::fmt::Display;
use std::path::PathBuf;
use std::time::{Duration, Instant};
use std::{fs, mem, panic};

// TODO: This doesn't need linkme or anything
days!(day1, day2, day3, day4, day5, day6);

#[derive(clap::Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Day to run
    ///
    /// If not passed, all days are run in order
    day: Option<usize>,

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

        let runner = DAYS[day - 1].ok_or_else(|| format!("Day {day} not implemented"))?;

        runner(&input);
        return Ok(());
    }

    let overall_start = Instant::now();
    let mut first = true;
    for (i, runner) in DAYS.iter().enumerate() {
        let Some(runner) = runner else { continue; };
        let day = i + 1;

        if mem::replace(&mut first, false) {
            println!();
        }
        println!("Day {day}");
        let input_path = input_for_day(day, args.demo);
        let input = fs::read_to_string(input_path)?;

        runner(&input);
    }
    let total_time = overall_start.elapsed();
    println!();
    println!("Total time: {:.2?}", total_time);
    Ok(())
}

fn latest_day() -> usize {
    DAYS.iter().rposition(|d| d.is_some()).unwrap() + 1
}

fn input_for_day(day: usize, demo: bool) -> PathBuf {
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

macro_rules! days {
    ($($mod_name:ident),*) => {
        $(mod $mod_name;)*

        const DAYS: [Option<fn(&str)>; 25] = {
            let mut result: [Option<fn(&str)>; 25] = [None; 25];

            $(
            {
                fn run_day(s: &str) {
                    let (gen_elapsed, input) = $crate::time(|| $mod_name::generator(s));
                    let input = match input {
                        Ok(i) => i,
                        Err(e) => {
                            println!("Generator error: {e}");
                            return;
                        }
                    };
                    let (p1_elapsed, p1_result) = $crate::time(|| $mod_name::part_1(&input));
                    let (p2_elapsed, p2_result) = $crate::time(|| $mod_name::part_2(&input));

                    let p1_result = $crate::stringify_res(p1_result);
                    let p2_result = $crate::stringify_res(p2_result);

                    println!("Gen    ({:.2?})", gen_elapsed);
                    println!("Part 1 ({:.2?}) {p1_result}", p1_elapsed);
                    println!("Part 2 ({:.2?}) {p2_result}", p2_elapsed);
                    println!("Total  ({:.2?})", gen_elapsed + p1_elapsed + p2_elapsed);
                }

                result[$crate::extract_day_number(stringify!($mod_name)) as usize - 1] = Some(run_day);
            }
            )*

            result
        };
    };
}
use days;

macro_rules! day_test {
    (demo_1 == $result:expr) => {
        #[test]
        fn test_demo_1() {
            let input = $crate::day_test!(@demo_input);
            let input = generator(&input);
            assert_eq!(part_1(&input), $result);
        }
    };
    (demo_2 == $result:expr) => {
        #[test]
        fn test_demo_2() {
            let input = $crate::day_test!(@demo_input);
            let input = generator(&input);
            assert_eq!(part_2(&input), $result);
        }
    };
    (part_1 == $result:expr) => {
        #[test]
        fn test_part_1() {
            let input = $crate::day_test!(@real_input);
            let input = generator(&input);
            assert_eq!(part_1(&input), $result);
        }
    };
    (part_2 == $result:expr) => {
        #[test]
        fn test_part_2() {
            let input = $crate::day_test!(@real_input);
            let input = generator(&input);
            assert_eq!(part_2(&input), $result);
        }
    };
    (@demo_input) => {
        {
            let day_num = $crate::extract_day_number(module_path!());
            std::fs::read_to_string(&format!("input/2022/demo{day_num}.txt")).unwrap()
        }
    };
    (@real_input) => {
        {
            let day_num = $crate::extract_day_number(module_path!());
            std::fs::read_to_string(&format!("input/2022/day{day_num}.txt")).unwrap()
        }
    };
}
use day_test;

const fn extract_day_number(s: &str) -> u32 {
    let mut day_number = 0;
    let s = s.as_bytes();

    let mut i = s.len();
    loop {
        if s[i - 3] == b'd' && s[i - 2] == b'a' && s[i - 1] == b'y' {
            break;
        }
        i -= 1;
    }

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
