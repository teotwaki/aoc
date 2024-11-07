use common::Answer;
use std::{collections::HashSet, str::FromStr};

#[derive(Default)]
struct Head {
    x: i16,
    y: i16,
}

impl Head {
    fn up(&mut self) {
        self.y -= 1;
    }

    fn down(&mut self) {
        self.y += 1;
    }

    fn left(&mut self) {
        self.x -= 1;
    }

    fn right(&mut self) {
        self.x += 1;
    }

    fn move_towards(&mut self, dir: &Direction) -> (i16, i16) {
        match dir {
            Direction::Up => self.up(),
            Direction::Down => self.down(),
            Direction::Left => self.left(),
            Direction::Right => self.right(),
        }

        (self.x, self.y)
    }
}

#[derive(Default, Clone)]
struct Knot {
    x: i16,
    y: i16,
    coords: HashSet<(i16, i16)>,
}

impl Knot {
    fn move_needed(&self, coords: (i16, i16)) -> bool {
        (coords.0 - self.x).abs() == 2 || (coords.1 - self.y).abs() == 2
    }

    fn move_towards(&mut self, dest: (i16, i16)) -> (i16, i16) {
        if self.move_needed(dest) {
            match dest.0 - self.x {
                2 | 1 => self.x += 1,
                -2 | -1 => self.x -= 1,
                _ => {}
            };
            match dest.1 - self.y {
                2 | 1 => self.y += 1,
                -2 | -1 => self.y -= 1,
                _ => {}
            };
        }

        self.coords.insert((self.x, self.y));

        (self.x, self.y)
    }

    fn total_unique_locations(&self) -> usize {
        self.coords.len()
    }
}

enum Direction {
    Up,
    Down,
    Left,
    Right,
}

struct Motion {
    pub direction: Direction,
    pub count: i16,
}

impl FromStr for Motion {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts: Vec<&str> = s.split(' ').collect();

        let count = parts[1].parse().unwrap();

        let direction = match parts[0] {
            "U" => Direction::Up,
            "D" => Direction::Down,
            "L" => Direction::Left,
            "R" => Direction::Right,
            _ => unreachable!(),
        };

        Ok(Motion { direction, count })
    }
}

fn calculate_knots(s: &str) -> Vec<Knot> {
    let mut head: Head = Default::default();
    let mut knots: Vec<Knot> = vec![Default::default(); 9];

    s.lines()
        .map(|l| l.parse().expect("Couldn't parse line as Motion"))
        .for_each(|m: Motion| {
            for _ in 0..m.count {
                let mut coords = head.move_towards(&m.direction);
                coords = knots[0].move_towards(coords);
                knots
                    .iter_mut()
                    .skip(1)
                    .for_each(|k| coords = k.move_towards(coords));
            }
        });

    knots
}

pub fn step1(s: &str) -> Answer {
    let knots = calculate_knots(s);

    knots[0].total_unique_locations().into()
}

pub fn step2(s: &str) -> Answer {
    let knots = calculate_knots(s);

    knots[8].total_unique_locations().into()
}
