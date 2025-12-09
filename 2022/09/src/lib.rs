use common::{Answer, Coordinates, Direction};
use rustc_hash::FxHashSet;
use std::str::FromStr;

type Coords = Coordinates<i16>;

#[derive(Default, Clone)]
struct Knot {
    current: Coords,
    coords: FxHashSet<Coords>,
}

impl Knot {
    fn move_needed(&self, dest: Coords) -> bool {
        dest.x().abs_diff(self.current.x()) == 2 || dest.y().abs_diff(self.current.y()) == 2
    }

    fn move_towards(&mut self, dest: Coords) -> Coords {
        if self.move_needed(dest) {
            match dest.x() - self.current.x() {
                2 | 1 => {
                    self.current.move_right();
                }
                -2 | -1 => {
                    self.current.move_left();
                }
                _ => {}
            };
            match dest.y() - self.current.y() {
                2 | 1 => {
                    self.current.move_down();
                }
                -2 | -1 => {
                    self.current.move_up();
                }
                _ => {}
            };
        }

        self.coords.insert(self.current);

        self.current
    }

    fn total_unique_locations(&self) -> usize {
        self.coords.len()
    }
}

struct Motion {
    pub direction: Direction,
    pub count: i16,
}

impl FromStr for Motion {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use Direction::*;

        let parts: Vec<&str> = s.split(' ').collect();
        let count = parts[1].parse().unwrap();

        let direction = match parts[0] {
            "U" => Up,
            "D" => Down,
            "L" => Left,
            "R" => Right,
            _ => unreachable!(),
        };

        Ok(Motion { direction, count })
    }
}

fn calculate_knots(s: &str) -> Vec<Knot> {
    let mut head: Coords = Default::default();
    let mut knots: Vec<Knot> = vec![Default::default(); 9];

    s.lines()
        .map(|l| l.parse().expect("Couldn't parse line as Motion"))
        .for_each(|m: Motion| {
            for _ in 0..m.count {
                let mut coords = *head.move_next(m.direction);
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
