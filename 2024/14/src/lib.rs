use common::{Answer, Coordinates};
use num::Integer;
use rustc_hash::FxHashSet;

type IntType = i32;
type Coords = Coordinates<IntType>;

#[derive(Debug, Clone, Copy)]
struct Robot {
    pos: Coords,
    movement: Coords,
}

impl Robot {
    fn simulate(&mut self, n: IntType, field: Coords) {
        let mut x = (self.pos.x() + self.movement.x() * n) % field.x();
        let mut y = (self.pos.y() + self.movement.y() * n) % field.y();

        if x < 0 {
            x += field.x();
        }

        if y < 0 {
            y += field.y();
        }

        self.pos = (x, y).into();
    }
}

fn parse_pair(s: &str) -> Coords {
    s.split('=')
        .nth(1)
        .map(|s| {
            let mut parts = s.split(',');

            let x = parts
                .next()
                .and_then(|s| s.parse::<IntType>().ok())
                .unwrap();
            let y = parts
                .next()
                .and_then(|s| s.parse::<IntType>().ok())
                .unwrap();

            (x, y).into()
        })
        .unwrap()
}

fn parse(s: &str) -> impl Iterator<Item = Robot> + '_ {
    s.lines().map(|l| {
        let mut numbers = l.split_whitespace().map(parse_pair);

        Robot {
            pos: numbers.next().unwrap(),
            movement: numbers.next().unwrap(),
        }
    })
}

fn quadrants(field: Coords) -> [(Coords, Coords); 4] {
    let (half_x, diff_x) = field.x().div_rem(&2);
    let (half_y, diff_y) = field.y().div_rem(&2);

    [
        ((0, 0).into(), (half_x, half_y).into()),
        ((half_x + diff_x, 0).into(), (field.x(), half_y).into()),
        ((0, half_y + diff_y).into(), (half_x, field.y()).into()),
        (
            (half_x + diff_x, half_y + diff_y).into(),
            (field.x(), field.y()).into(),
        ),
    ]
}

fn filter_quadrant(quadrant: (Coords, Coords), pos: Coords) -> bool {
    pos.x() >= quadrant.0.x()
        && pos.x() < quadrant.1.x()
        && pos.y() >= quadrant.0.y()
        && pos.y() < quadrant.1.y()
}

fn calculate_safety_factor(
    robots: impl Iterator<Item = Robot>,
    seconds: IntType,
    field: Coords,
) -> usize {
    let robots = robots
        .map(|mut r| {
            r.simulate(seconds, field);
            r
        })
        .collect::<Vec<_>>();
    let quadrants = quadrants(field);

    (0..4)
        .map(|i| {
            robots
                .iter()
                .filter(|r| filter_quadrant(quadrants[i], r.pos))
                .count()
                .max(1)
        })
        .product()
}

pub fn step1(s: &str) -> Answer {
    let field = (101, 103).into();
    calculate_safety_factor(parse(s), 100, field).into()
}

pub fn step2(s: &str) -> Answer {
    let field = (101, 103).into();
    let mut robots = parse(s).collect::<Vec<_>>();

    let mut count = 1;
    loop {
        robots.iter_mut().for_each(|r| r.simulate(1, field));
        let hash = robots.iter().map(|r| r.pos).collect::<FxHashSet<_>>();

        if robots.len() == hash.len() {
            break;
        }

        count += 1;
    }

    count.into()
}

#[cfg(test)]
mod test_2024_14 {
    use super::*;

    const INPUT: &str = r#"p=0,4 v=3,-3
p=6,3 v=-1,-3
p=10,3 v=-1,2
p=2,0 v=2,-1
p=0,0 v=1,3
p=3,0 v=-2,-2
p=7,6 v=-1,-3
p=3,0 v=-1,-2
p=9,3 v=2,3
p=7,3 v=-1,2
p=2,4 v=2,-3
p=9,5 v=-3,-3"#;

    #[test]
    fn calculate_safety_factor_finds_example_value() {
        let robots = parse(INPUT);
        let field = (11, 7).into();
        assert_eq!(calculate_safety_factor(robots, 5, field), 12);
    }

    #[test]
    fn robot_simulate_moves_correctly() {
        let field = (11, 7).into();
        let mut robot = Robot {
            pos: (2, 4).into(),
            movement: (2, -3).into(),
        };

        robot.simulate(1, field);

        assert_eq!(robot.pos, (4, 1).into());

        robot.simulate(1, field);

        assert_eq!(robot.pos, (6, 5).into());
    }
}
