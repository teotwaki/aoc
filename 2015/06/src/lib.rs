use common::{Answer, Coordinates, Grid};
use regex::Regex;
use std::sync::LazyLock;

type IntType = u16;
type Lights = Grid<IntType, bool>;
type DimmableLights = Grid<IntType, i32>;
type Coords = Coordinates<IntType>;

enum Instruction {
    On(Coords, Coords),
    Off(Coords, Coords),
    Toggle(Coords, Coords),
}

fn parse(s: &str) -> impl Iterator<Item = Instruction> + '_ {
    static EXPRESSION: &str =
        r"(turn on|turn off|toggle) (\d{1,3}),(\d{1,3}) through (\d{1,3}),(\d{1,3})";
    static RE: LazyLock<Regex> = LazyLock::new(|| Regex::new(EXPRESSION).unwrap());

    RE.captures_iter(s).map(|caps| {
        let (_, [action, x1, y1, x2, y2]) = caps.extract();
        let (x1, y1, x2, y2) = (
            x1.parse().unwrap(),
            y1.parse().unwrap(),
            x2.parse().unwrap(),
            y2.parse().unwrap(),
        );

        match action {
            "turn on" => Instruction::On(Coordinates::new(x1, y1), Coordinates::new(x2, y2)),
            "turn off" => Instruction::Off(Coordinates::new(x1, y1), Coordinates::new(x2, y2)),
            "toggle" => Instruction::Toggle(Coordinates::new(x1, y1), Coordinates::new(x2, y2)),
            _ => unreachable!(),
        }
    })
}

#[inline]
fn turn_on(lights: &mut Lights, pos: Coords) {
    lights.store(pos, true);
}

#[inline]
fn turn_off(lights: &mut Lights, pos: Coords) {
    lights.store(pos, false);
}

#[inline]
fn toggle(lights: &mut Lights, pos: Coords) {
    let value = *lights.get(&pos).unwrap_or(&false);
    lights.store(pos, !value);
}

#[inline]
fn count_lights(lights: &Lights) -> usize {
    lights.iter().filter(|(_, b)| **b).count()
}

pub fn step1(s: &str) -> Answer {
    let mut lights = Lights::new();

    parse(s).for_each(|i| match i {
        Instruction::On(a, b) => a.range_inclusive(b).for_each(|p| turn_on(&mut lights, p)),
        Instruction::Off(a, b) => a.range_inclusive(b).for_each(|p| turn_off(&mut lights, p)),
        Instruction::Toggle(a, b) => a.range_inclusive(b).for_each(|p| toggle(&mut lights, p)),
    });

    count_lights(&lights).into()
}

#[inline]
fn increase(lights: &mut DimmableLights, pos: Coords) {
    *lights.get_mut(pos) += 1;
}

#[inline]
fn decrease(lights: &mut DimmableLights, pos: Coords) {
    let value = lights.get_mut(pos);

    if *value > 0 {
        *value -= 1;
    }
}

#[inline]
fn really_increase(lights: &mut DimmableLights, pos: Coords) {
    *lights.get_mut(pos) += 2;
}

#[inline]
fn sum_brightness(lights: &DimmableLights) -> i32 {
    lights.iter().map(|(_, l)| l).sum()
}

pub fn step2(s: &str) -> Answer {
    let mut lights = DimmableLights::new();

    parse(s).for_each(|i| match i {
        Instruction::On(a, b) => a.range_inclusive(b).for_each(|p| increase(&mut lights, p)),
        Instruction::Off(a, b) => a.range_inclusive(b).for_each(|p| decrease(&mut lights, p)),
        Instruction::Toggle(a, b) => a
            .range_inclusive(b)
            .for_each(|p| really_increase(&mut lights, p)),
    });

    sum_brightness(&lights).into()
}

#[cfg(test)]
mod test_2015_06 {
    use super::*;

    #[test]
    fn dimmable_lights_store_correct_value_after_increase() {
        let mut lights = DimmableLights::new();
        increase(&mut lights, (1, 1).into());

        assert_eq!(sum_brightness(&lights), 1);
    }

    #[test]
    fn dimmable_lights_store_correct_value_after_decrease() {
        let mut lights = DimmableLights::new();
        increase(&mut lights, (1, 1).into());
        decrease(&mut lights, (1, 1).into());

        assert_eq!(sum_brightness(&lights), 0);
    }

    #[test]
    fn dimmable_lights_store_correct_value_after_real_increase() {
        let mut lights = DimmableLights::new();
        really_increase(&mut lights, (1, 1).into());

        assert_eq!(sum_brightness(&lights), 2);
    }

    #[test]
    fn dimmable_lights_store_correct_value_after_multiple_changes() {
        let mut lights = DimmableLights::new();
        really_increase(&mut lights, (1, 1).into());
        increase(&mut lights, (1, 1).into());
        decrease(&mut lights, (1, 1).into());
        really_increase(&mut lights, (1, 1).into());

        assert_eq!(sum_brightness(&lights), 4);
    }

    #[test]
    fn step2_handles_first_example() {
        let answer = step2("turn on 0,0 through 0,0");

        assert_eq!(answer, Answer::Signed(1));
    }

    #[test]
    fn step2_handles_second_example() {
        let answer = step2("toggle 0,0 through 999,999");

        assert_eq!(answer, Answer::Signed(2_000_000));
    }
}
