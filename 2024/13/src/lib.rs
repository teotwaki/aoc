use common::{Answer, Coordinates};
use num::Integer;

type IntType = i64;
type Coords = Coordinates<IntType>;

#[derive(Debug, Clone, Copy)]
struct ClawMachine {
    prize: Coords,
    a: Coords,
    b: Coords,
}

impl ClawMachine {
    fn solve(&self) -> Option<(IntType, IntType)> {
        let det = self.a.x() * self.b.y() - self.a.y() * self.b.x();
        let det_a = self.prize.x() * self.b.y() - self.prize.y() * self.b.x();
        let det_b = self.prize.y() * self.a.x() - self.prize.x() * self.a.y();

        let (a_presses, a_rem) = det_a.div_rem(&det);
        let (b_presses, b_rem) = det_b.div_rem(&det);

        if a_rem == 0 && b_rem == 0 {
            Some((a_presses, b_presses))
        } else {
            None
        }
    }

    fn correct_units(mut self) -> Self {
        self.prize = (
            self.prize.x() + 10_000_000_000_000,
            self.prize.y() + 10_000_000_000_000,
        )
            .into();

        self
    }
}

fn parse_line(s: &str, c: char) -> Coords {
    let mut parts = s.split(c);
    let x = parts
        .nth(1)
        .and_then(|s| s.find(',').and_then(|pos| s[..pos].parse::<IntType>().ok()))
        .unwrap();

    let y = parts
        .next()
        .and_then(|s| s.parse::<IntType>().ok())
        .unwrap();

    (x, y).into()
}

fn parse(s: &str) -> impl Iterator<Item = ClawMachine> + '_ {
    s.split("\n\n").map(|description| {
        let mut lines = description.lines();

        ClawMachine {
            a: parse_line(lines.next().unwrap(), '+'),
            b: parse_line(lines.next().unwrap(), '+'),
            prize: parse_line(lines.next().unwrap(), '='),
        }
    })
}

pub fn step1(s: &str) -> Answer {
    parse(s)
        .filter_map(|machine| machine.solve())
        .map(|(a, b)| a * 3 + b)
        .sum::<IntType>()
        .into()
}

pub fn step2(s: &str) -> Answer {
    parse(s)
        .map(|machine| machine.correct_units())
        .filter_map(|machine| machine.solve())
        .map(|(a, b)| a * 3 + b)
        .sum::<IntType>()
        .into()
}
