use anyhow::Result;
use aoc_workbench::{Day, Registry, Year};

use clap::Parser;

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

    add!(2015, 1, 2, 3);
    add!(2022, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15);
    add!(2023, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 13, 14, 15, 16);
    add!(2024, 1, 2, 3, 4);

    let cli = Cli::parse();
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
