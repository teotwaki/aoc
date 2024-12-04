use common::{distances::manhattan, Answer, Coordinates};
use itertools::Itertools;
use regex::Regex;
use std::sync::LazyLock;

type IntType = i64;
type Coord = Coordinates<IntType>;

#[derive(Debug, Copy, Clone)]
struct SensorReadout {
    sensor: Coord,
    beacon: Coord,
    distance: IntType,
}

impl SensorReadout {
    #[inline]
    fn new(s: Coord, b: Coord) -> Self {
        Self {
            sensor: s,
            beacon: b,
            distance: manhattan(s, b),
        }
    }

    #[inline]
    fn in_range(&self, pos: Coord) -> bool {
        manhattan(self.sensor, pos) <= self.distance
    }

    fn in_range_in_row(&self, y: IntType) -> Vec<Coord> {
        let y_diff = (self.sensor.y() - y).abs();

        if y_diff <= self.distance {
            let x_range = self.distance - y_diff;
            let min_x = self.sensor.x() - x_range;
            let max_x = self.sensor.x() + x_range;

            (min_x..=max_x).map(|x| Coordinates::new(x, y)).collect()
        } else {
            vec![]
        }
    }

    fn perimeter(&self) -> impl Iterator<Item = Coord> {
        let distance = self.distance + 1;
        let sx = self.sensor.x();
        let sy = self.sensor.y();

        let mut points = Vec::new();

        // Top-right edge
        for i in 0..=distance {
            points.push(Coordinates::new(sx + i, sy - distance + i));
        }
        // Bottom-right edge
        for i in 0..=distance {
            points.push(Coordinates::new(sx + distance - i, sy + i));
        }
        // Bottom-left edge
        for i in 0..=distance {
            points.push(Coordinates::new(sx - i, sy + distance - i));
        }
        // Top-left edge
        for i in 0..=distance {
            points.push(Coordinates::new(sx - distance + i, sy - i));
        }

        // Deduplicate and filter points within bounds
        points
            .into_iter()
            .filter(|p| p.x() >= 0 && p.x() < 4_000_000 && p.y() >= 0 && p.y() < 4_000_000)
            .unique() // Ensure unique points
    }
}

#[derive(Debug, Clone)]
struct Map {
    readouts: Vec<SensorReadout>,
    beacons: Vec<Coord>,
}

impl Map {
    fn new(s: &str) -> Self {
        let readouts = s.lines().map(extract_data).collect::<Vec<_>>();
        let beacons = readouts.iter().map(|r| r.beacon).collect();

        Self { readouts, beacons }
    }

    fn unavailable_locations_in_row(&self, row: IntType) -> usize {
        let unavailable_positions = self
            .readouts
            .iter()
            .flat_map(|s| s.in_range_in_row(row))
            .unique()
            .filter(|p| !self.beacons.contains(p))
            .count();

        unavailable_positions
    }

    fn beacon_location(&self, limit: IntType) -> Coord {
        self.readouts
            .iter()
            .flat_map(|s| s.perimeter())
            .unique()
            .filter(|p| p.x() >= 0 && p.x() < limit && p.y() >= 0 && p.y() < limit)
            .find(|&p| self.readouts.iter().all(|s| !s.in_range(p)))
            .unwrap()
    }
}

fn extract_data(s: &str) -> SensorReadout {
    static RE: LazyLock<Regex> = LazyLock::new(|| {
        Regex::new(
            r"^Sensor at x=(?P<sx>[^,]+), y=(?P<sy>[^:]+): closest beacon is at x=(?P<bx>[^,]+), y=(?P<by>.+)$"
        )
        .expect("Invalid regex")
    });

    RE.captures(s)
        .map(|c| {
            (
                c.name("sx").and_then(|m| m.as_str().parse().ok()).unwrap(),
                c.name("sy").and_then(|m| m.as_str().parse().ok()).unwrap(),
                c.name("bx").and_then(|m| m.as_str().parse().ok()).unwrap(),
                c.name("by").and_then(|m| m.as_str().parse().ok()).unwrap(),
            )
        })
        .map(|(sx, sy, bx, by)| {
            SensorReadout::new(Coordinates::new(sx, sy), Coordinates::new(bx, by))
        })
        .unwrap()
}

pub fn step1(s: &str) -> Answer {
    let map = Map::new(s);

    map.unavailable_locations_in_row(2_000_000).into()
}

fn frequency(x: IntType, y: IntType) -> IntType {
    x * 4_000_000 + y
}

pub fn step2(s: &str) -> Answer {
    let map = Map::new(s);

    let pos = map.beacon_location(4_000_000);

    frequency(pos.x(), pos.y()).into()
}

#[cfg(test)]
mod test {
    use super::*;

    const SMALL_INPUT: &str = r#"Sensor at x=2, y=18: closest beacon is at x=-2, y=15
Sensor at x=9, y=16: closest beacon is at x=10, y=16
Sensor at x=13, y=2: closest beacon is at x=15, y=3
Sensor at x=12, y=14: closest beacon is at x=10, y=16
Sensor at x=10, y=20: closest beacon is at x=10, y=16
Sensor at x=14, y=17: closest beacon is at x=10, y=16
Sensor at x=8, y=7: closest beacon is at x=2, y=10
Sensor at x=2, y=0: closest beacon is at x=2, y=10
Sensor at x=0, y=11: closest beacon is at x=2, y=10
Sensor at x=20, y=14: closest beacon is at x=25, y=17
Sensor at x=17, y=20: closest beacon is at x=21, y=22
Sensor at x=16, y=7: closest beacon is at x=15, y=3
Sensor at x=14, y=3: closest beacon is at x=15, y=3
Sensor at x=20, y=1: closest beacon is at x=15, y=3"#;

    #[test]
    fn step1() {
        let map = Map::new(SMALL_INPUT);

        assert_eq!(map.unavailable_locations_in_row(10), 26)
    }

    #[test]
    fn beacon_location() {
        let map = Map::new(SMALL_INPUT);
        let pos = map.beacon_location(20);

        assert_eq!(pos, Coordinates::new(14, 11));
    }

    #[test]
    fn test_step2() {
        let map = Map::new(SMALL_INPUT);
        let pos = map.beacon_location(20);
        let freq = frequency(pos.x(), pos.y());

        assert_eq!(freq, 56000011);
    }
}
