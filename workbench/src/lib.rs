use common::Answer;
use std::collections::HashMap;
use thiserror::Error;

pub type Solver = fn(&str) -> Answer;

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub struct Year(u16);

#[derive(Error, Debug)]
#[error("invalid year number: {0}")]
pub struct InvalidYear(u16);

impl Year {
    pub fn new(y: u16) -> Result<Self, InvalidYear> {
        match y {
            2015..=2030 => Ok(Self(y)),
            _ => Err(InvalidYear(y)),
        }
    }
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub struct Day(u8);

#[derive(Error, Debug)]
#[error("invalid day number: {0}")]
pub struct InvalidDay(u8);

impl Day {
    pub fn new(d: u8) -> Result<Self, InvalidDay> {
        match d {
            1..=25 => Ok(Self(d)),
            _ => Err(InvalidDay(d)),
        }
    }
}

#[derive(Debug, Clone, Copy, Hash)]
pub struct Solution {
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
pub struct Registry(HashMap<Year, HashMap<Day, Solution>>);

impl Registry {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn add(&mut self, y: Year, d: Day, s: Solution) {
        self.0.entry(y).or_default().entry(d).or_insert(s);
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn year_can_be_initialised() {
        assert!(Year::new(2015).is_ok());
        assert!(Year::new(2030).is_ok());
    }

    #[test]
    fn year_rejects_invalid_values() {
        assert!(Year::new(0).is_err());
        assert!(Year::new(2014).is_err());
        assert!(Year::new(2031).is_err());
    }

    #[test]
    fn day_can_be_initialised() {
        assert!(Day::new(1).is_ok());
        assert!(Day::new(25).is_ok());
    }

    #[test]
    fn day_rejects_invalid_values() {
        assert!(Day::new(0).is_err());
        assert!(Day::new(26).is_err());
    }
}
