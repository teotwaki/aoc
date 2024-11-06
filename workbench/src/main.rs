use anyhow::Result;
use aoc_workbench::{Day, Registry, Solution, Year};

fn main() -> Result<()> {
    let mut registry = Registry::new();

    registry.add(
        Year::new(2023)?,
        Day::new(1)?,
        Solution::both(y2023_d01::part1, y2023_d01::part2),
    );

    Ok(())
}
