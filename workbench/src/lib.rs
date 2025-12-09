use common::Answer;
use itertools::Itertools;
use rustc_hash::FxHashMap;
use std::{fmt::Display, fs::read_to_string, num::ParseIntError, str::FromStr};
use thiserror::Error;

pub type Solver = fn(&str) -> Answer;

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct Year(u16);

#[derive(Error, Debug)]
#[error("expected year between 2015 and 2030")]
pub struct InvalidYear;

impl Year {
    pub fn new(y: u16) -> Result<Self, InvalidYear> {
        match y {
            2015..=2030 => Ok(Self(y)),
            _ => Err(InvalidYear {}),
        }
    }
}

impl Display for Year {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}

#[derive(Error, Debug)]
pub enum YearFromStrError {
    #[error("invalid year: {0}")]
    InvalidYear(#[from] InvalidYear),

    #[error("invalid integer: {0}")]
    ParseInt(#[from] ParseIntError),
}

impl FromStr for Year {
    type Err = YearFromStrError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self::new(s.parse()?)?)
    }
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct Day(u8);

#[derive(Error, Debug)]
#[error("expected day between 1 and 25")]
pub struct InvalidDay;

impl Day {
    pub fn new(d: u8) -> Result<Self, InvalidDay> {
        match d {
            1..=25 => Ok(Self(d)),
            _ => Err(InvalidDay {}),
        }
    }
}

impl Display for Day {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}

#[derive(Error, Debug)]
pub enum DayFromStrError {
    #[error("invalid day: {0}")]
    InvalidYear(#[from] InvalidDay),

    #[error("invalid integer: {0}")]
    ParseInt(#[from] ParseIntError),
}

impl FromStr for Day {
    type Err = DayFromStrError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self::new(s.parse()?)?)
    }
}

#[derive(Debug, Clone, Copy, Hash)]
pub struct Solution {
    step1: Option<Solver>,
    step2: Option<Solver>,
}

impl Solution {
    pub fn new(first: Solver, second: Solver) -> Self {
        Solution {
            step1: Some(first),
            step2: Some(second),
        }
    }
}

#[derive(Debug, Clone, Default)]
pub struct Registry(FxHashMap<Year, FxHashMap<Day, Solution>>);

#[derive(Error, Debug)]
pub enum RunError {
    #[error("there is no solution implemented for this day")]
    MissingSolution,

    #[error("unable to open input: {0}")]
    InputError(#[from] std::io::Error),
}

impl Registry {
    #[inline]
    pub fn new() -> Self {
        Default::default()
    }

    pub fn add(&mut self, y: Year, d: Day, s: Solution) {
        self.0.entry(y).or_default().entry(d).or_insert(s);
    }

    pub fn latest_year(&self) -> Year {
        self.0.keys().sorted_unstable().last().copied().unwrap()
    }

    pub fn latest_day(&self, y: Year) -> Day {
        *self
            .0
            .get(&y)
            .and_then(|days| days.keys().sorted().last())
            .unwrap()
    }

    fn get_input(&self, y: Year, d: Day) -> Result<String, std::io::Error> {
        read_to_string(format!("../{y}/{:02}/input.txt", d))
    }

    pub fn run_step1(&self, y: Year, d: Day) -> Result<Answer, RunError> {
        Ok(self
            .0
            .get(&y)
            .ok_or(RunError::MissingSolution)?
            .get(&d)
            .ok_or(RunError::MissingSolution)?
            .step1
            .unwrap()(&self.get_input(y, d)?))
    }

    pub fn run_step2(&self, y: Year, d: Day) -> Result<Answer, RunError> {
        Ok(self
            .0
            .get(&y)
            .ok_or(RunError::MissingSolution)?
            .get(&d)
            .ok_or(RunError::MissingSolution)?
            .step2
            .unwrap()(&self.get_input(y, d)?))
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
