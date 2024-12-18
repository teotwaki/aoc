use anyhow::Result;
use aoc_workbench::{Day, Registry, Year};
use clap::Parser;
use humantime::format_duration;
use std::time::{Duration, Instant};

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Cli {
    #[arg(value_parser = clap::value_parser!(Year))]
    year: Option<Year>,

    #[arg(value_parser = clap::value_parser!(Day))]
    day: Option<Day>,

    #[arg(long)]
    step1: bool,

    #[arg(long)]
    step2: bool,

    #[arg(long)]
    bench: bool,
}

fn run(registry: &Registry, cli: &Cli) -> Result<()> {
    let year = cli.year.unwrap_or_else(|| registry.latest_year());
    let day = cli.day.unwrap_or_else(|| registry.latest_day(year));

    println!("Running solution for {year}-{day}");

    if cli.step1 || !cli.step2 {
        let answer = registry.run_step1(year, day)?;
        println!("Step 1 answer: {answer}");
    }

    if cli.step2 || !cli.step1 {
        let answer = registry.run_step2(year, day)?;
        println!("Step 2 answer: {answer}");
    }

    Ok(())
}

fn bench<F: Fn() -> Result<()>>(f: F) -> Result<()> {
    let run = || -> Result<Duration> {
        let now = Instant::now();
        f()?;
        Ok(now.elapsed())
    };

    let now = Instant::now();
    while now.elapsed() < Duration::from_secs(1) {
        run()?;
    }

    let mut durations = Vec::with_capacity(2000);
    let now = Instant::now();

    while now.elapsed() < Duration::from_secs(10) {
        durations.push(run()?);
    }

    let average = durations.iter().sum::<Duration>() / durations.len() as u32;

    println!(
        "  - {} iterations, average: {}",
        durations.len(),
        format_duration(average),
    );

    Ok(())
}

fn benchmark(registry: &Registry, cli: &Cli) -> Result<()> {
    let year = cli.year.unwrap_or_else(|| registry.latest_year());
    let day = cli.day.unwrap_or_else(|| registry.latest_day(year));

    if cli.step1 || !cli.step2 {
        println!("Benchmarking solution for {year}-{day} part 1");

        bench(|| {
            registry.run_step1(year, day)?;
            Ok(())
        })?;
    }

    if cli.step2 || !cli.step1 {
        println!("Benchmarking solution for {year}-{day} part 2");

        bench(|| {
            registry.run_step2(year, day)?;
            Ok(())
        })?;
    }

    Ok(())
}

fn main() -> Result<()> {
    let mut registry = Registry::new();

    macro_rules! add {
        ($y:literal, $d:literal) => {
            paste::paste! {
                registry.add(
                    aoc_workbench::Year::new($y)?,
                    aoc_workbench::Day::new($d)?,
                    aoc_workbench::Solution::both([< y $y _d $d >]::step1, [< y $y _d $d >]::step2));
            }
        };
        ($y:literal, $d:literal, $($days:literal),+) => {
            add!($y, $d);
            add!($y, $($days),+);
        };
    }

    add!(2015, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21);
    add!(2022, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15);
    add!(2023, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 13, 14, 15, 16);
    add!(2024, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 16, 17, 18);

    let cli = Cli::parse();

    if !cli.bench {
        run(&registry, &cli)?;
    } else {
        benchmark(&registry, &cli)?;
    }

    Ok(())
}
