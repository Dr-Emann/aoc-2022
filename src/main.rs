use aoc_2022::DAYS;
use clap::Parser;
use std::path::PathBuf;
use std::time::Instant;
use std::{fs, mem};

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

        runner(&input, true);
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

        runner(&input, true);
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
