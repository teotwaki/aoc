use common::Answer;
use std::collections::HashMap;

type Solver = fn(&str) -> Answer;

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
struct Year(u16);

impl Year {
    pub fn new(y: u16) -> Self {
        Self(y)
    }
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
struct Day(u8);

impl Day {
    pub fn new(d: u8) -> Self {
        Self(d)
    }
}

#[derive(Debug, Clone, Copy, Hash)]
struct Solution {
    part1: Option<Solver>,
    part2: Option<Solver>,
}

impl Solution {
    pub fn first(s: Solver) -> Self {
        Solution {
            part1: Some(s),
            part2: None,
        }
    }

    pub fn second(s: Solver) -> Self {
        Solution {
            part1: None,
            part2: Some(s),
        }
    }

    pub fn both(first: Solver, second: Solver) -> Self {
        Solution {
            part1: Some(first),
            part2: Some(second),
        }
    }
}

#[derive(Debug, Clone, Default)]
struct Registry(HashMap<Year, HashMap<Day, Solution>>);

impl Registry {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn add(&mut self, y: Year, d: Day, s: Solution) {
        self.0.entry(y).or_default().entry(d).or_insert(s);
    }
}

fn main() {
    let mut registry = Registry::new();

    registry.add(
        Year::new(2023),
        Day::new(1),
        Solution::both(y2023_d01::part1, y2023_d01::part2),
    );
}
