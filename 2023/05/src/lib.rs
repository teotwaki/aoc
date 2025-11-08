use common::Answer;
use rayon::prelude::*;
use std::str::FromStr;

#[derive(Debug)]
struct Range {
    source_start: u32,
    destination_start: u32,
    length: u32,
}

impl FromStr for Range {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let v: Vec<_> = s.split(' ').map(str::parse).map(|f| f.unwrap()).collect();

        Ok(Range {
            destination_start: v[0],
            source_start: v[1],
            length: v[2],
        })
    }
}

impl Range {
    fn transpose(&self, value: u32) -> Option<u32> {
        if value >= self.source_start && value < (self.source_start + self.length) {
            if self.source_start > self.destination_start {
                Some(value - (self.source_start - self.destination_start))
            } else {
                Some(value + (self.destination_start - self.source_start))
            }
        } else {
            None
        }
    }
}

#[derive(Debug)]
struct Map {
    ranges: Vec<Range>,
}

impl FromStr for Map {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let ranges = s
            .lines()
            .filter_map(|l| {
                if l.contains(':') {
                    None
                } else {
                    Some(Range::from_str(l).unwrap())
                }
            })
            .collect();

        Ok(Map { ranges })
    }
}

impl Map {
    fn transpose(&self, value: u32) -> Option<u32> {
        self.ranges
            .iter()
            .map(|r| r.transpose(value))
            .find(|v| v.is_some())
            .unwrap_or(None)
    }
}

#[derive(Debug, Default)]
struct Almanac {
    seeds: Vec<u32>,
    maps: Vec<Map>,
}

impl FromStr for Almanac {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut seeds = vec![];

        let maps = s
            .split("\n\n")
            .filter_map(|part| {
                if part.contains('\n') {
                    Some(Map::from_str(part).unwrap())
                } else {
                    seeds = part[7..].split(' ').map(|s| s.parse().unwrap()).collect();
                    None
                }
            })
            .collect();

        Ok(Almanac { seeds, maps })
    }
}

impl Almanac {
    fn find_locations(&self) -> Vec<u32> {
        self.seeds.iter().map(|s| self.transpose(*s)).collect()
    }

    fn find_all_locations(&self) -> Vec<u32> {
        self.seeds
            .par_chunks(2)
            .flat_map(|c| (c[0]..(c[0] + c[1])).collect::<Vec<_>>())
            .map(|s| self.transpose(s))
            .collect()
    }

    fn transpose(&self, seed: u32) -> u32 {
        self.maps
            .iter()
            .fold(seed, |v, m| m.transpose(v).unwrap_or(v))
    }
}

pub fn step1(s: &str) -> Answer {
    let almanac = Almanac::from_str(s).unwrap();
    let lowest_location = *almanac.find_locations().iter().min().unwrap();

    lowest_location.into()
}

pub fn step2(s: &str) -> Answer {
    let almanac = Almanac::from_str(s).unwrap();
    let lowest_location = *almanac.find_all_locations().par_iter().min().unwrap();

    lowest_location.into()
}
