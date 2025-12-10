use common::Answer;
use itertools::Itertools;

type IntType = u16;

fn parse_target(s: &str) -> (IntType, usize) {
    let s = &s[1..s.len() - 1];
    let mut val = 0;

    s.chars()
        .enumerate()
        .filter(|(_, c)| *c == '#')
        .for_each(|(i, _)| val += 1 << i);

    (val, s.len())
}

fn parse_button(s: &str, target_length: usize) -> IntType {
    let s = &s[1..s.len() - 1];
    let mut val = 0;

    s.split(',')
        .map(|c| c.parse::<usize>().unwrap())
        .for_each(|i| val += 1 << i);

    val & ((1 << target_length) - 1)
}

fn parse_line(s: &str) -> (IntType, Vec<IntType>) {
    let parts = s.split_whitespace().collect::<Vec<_>>();

    let (target, lights_count) = parse_target(parts[0]);
    let buttons = parts[1..parts.len() - 1]
        .iter()
        .map(|s| parse_button(s, lights_count))
        .collect();

    (target, buttons)
}

fn parse(s: &str) -> Vec<(IntType, Vec<IntType>)> {
    s.lines().map(parse_line).collect()
}

pub fn step1(s: &str) -> Answer {
    let machines = parse(s);

    machines
        .iter()
        .map(|(target, buttons)| {
            buttons
                .iter()
                .powerset()
                .find(|set| set.iter().fold(0, |a, b| a ^ **b) == *target)
                .map(|set| set.len())
                .unwrap()
        })
        .sum::<usize>()
        .into()
}

pub fn step2(_: &str) -> Answer {
    ().into()
}

#[cfg(test)]
mod test {
    use super::*;
    use parameterized::parameterized;

    #[test]
    fn parse_target_finds_correct_value() {
        assert_eq!(parse_target("[.##.]"), (6, 4));
    }

    #[parameterized(
        input1 = { "(1,2)", "(0,1,2,3,4)", "(4)" },
        input2 = { 3, 5, 1 },
        result = { 6, 31, 0 },
    )]
    fn parse_button_finds_correct_value(input1: &str, input2: usize, result: IntType) {
        assert_eq!(parse_button(input1, input2), result);
    }

    const INPUT: &str = r#"[.##.] (3) (1,3) (2) (2,3) (0,2) (0,1) {3,5,4,7}
[...#.] (0,2,3,4) (2,3) (0,4) (0,1,2) (1,2,3,4) {7,5,12,7,2}
[.###.#] (0,1,2,3,4) (0,3,4) (0,1,2,4,5) (1,2) {10,11,11,5,10,5}"#;

    #[test]
    fn parse_extracts_correct_number_of_lines() {
        assert_eq!(parse(INPUT).len(), 3);
    }

    #[test]
    fn step1_computes_expected_sample_result() {
        assert_eq!(step1(INPUT), Answer::Unsigned(7));
    }

    /*
    #[test]
    fn step2_computes_expected_sample_result() {
        assert_eq!(step2(INPUT), Answer::Unsigned(5));
    }
    */
}
